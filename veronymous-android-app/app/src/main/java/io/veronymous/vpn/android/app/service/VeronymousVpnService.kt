package io.veronymous.vpn.android.app.service

import android.app.AlarmManager
import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.content.Context
import android.content.Intent
import android.net.IpPrefix
import android.net.VpnService
import android.os.Binder
import android.os.Build
import android.os.IBinder
import android.os.Parcel
import android.os.SystemClock
import android.util.Log
import androidx.activity.result.ActivityResultLauncher
import com.wireguard.android.backend.Backend
import com.wireguard.android.backend.GoBackend
import com.wireguard.android.backend.Tunnel
import com.wireguard.config.Config
import com.wireguard.config.InetEndpoint
import com.wireguard.config.InetNetwork
import com.wireguard.config.Interface
import com.wireguard.config.Peer
import com.wireguard.crypto.Key
import com.wireguard.crypto.KeyPair
import io.veronymous.android.veronymous.client.VeronymousClient
import io.veronymous.android.veronymous.client.config.VeronymousConfig
import io.veronymous.android.veronymous.client.listener.VeronymousTaskListener
import io.veronymous.android.veronymous.client.model.WireguardConnection
import io.veronymous.vpn.android.app.service.listener.ConnectionResultListener
import io.veronymous.vpn.android.app.state.VpnState
import io.veronymous.vpn.android.app.ui.activities.MainActivity
import io.veronymous.vpn.android.app.wg.VeronymousTunnel
import io.veronymous.client.exceptions.VeronymousClientException
import io.veronymous.vpn.android.app.R
import java.net.InetAddress
import java.time.Instant
import java.util.concurrent.Executors
import java.util.concurrent.TimeUnit

// TODO: Handle errors (e.g. stop and notify user when authentication is required)
class VeronymousVpnService : VpnService() {
    companion object {
        private val TAG = VeronymousVpnService::class.java.simpleName

        private const val START = "io.veronymous.android.vpn.app.service.VeronymousVpnService.START"

        private const val STOP = "io.veronymous.android.vpn.app.service.VeronymousVpnService.STOP"

        private const val REFRESH =
            "io.veronymous.android.vpn.app.service.VeronymousVpnService.REFRESH"

        private const val VPN_CONNECTION = "vpnConnection";

        const val SERVER_NAME = "serverName"

        private const val NOTIFICATION_ID = "VeronymousVpnService"

        const val VIEW_STATUS_REQUEST = 2

        const val REFRESH_CONNECTION_REQUEST = 3;

        const val DISCONNECTION_UPDATE =
            "io.veronymous.android.vpn.app.service.VeronymousVpnService.DISCONNECT"

        const val CONNECTION_UPDATE =
            "io.veronymous.android.vpn.app.service.VeronymousVpnService.CONNECTION"

        private val EXECUTOR = Executors.newSingleThreadExecutor();

        fun connect(
            context: Context,
            requestVpnPermission: ActivityResultLauncher<Intent>,
            serverName: String,
            resultListener: ConnectionResultListener
        ) {

            VeronymousClient.connect(context, serverName, object : VeronymousTaskListener<String> {
                override fun onResult(vpnConnection: String?) {
                    if (vpnConnection == null) {
                        Log.e(TAG, "Could not create VPN connection.")
                        return
                    }

                    Log.d(TAG, "Created vpn connection $vpnConnection")

                    // Prepare VPN service
                    if (!prepareVpnService(context, requestVpnPermission))
                        return

                    val serviceIntent = Intent(context, VeronymousVpnService::class.java)
                    serviceIntent.action = START
                    serviceIntent.putExtra(VPN_CONNECTION, vpnConnection)
                    serviceIntent.putExtra(SERVER_NAME, serverName)

                    context.startService(serviceIntent)

                    resultListener.onSuccess()
                }

                override fun onError(e: VeronymousClientException?) {
                    Log.d(TAG, "Could not create vpn connection", e)

                    resultListener.onFailure(e)
                }

            })
        }

        fun disconnect(context: Context) {
            val serviceIntent = Intent(context, VeronymousVpnService::class.java)
            serviceIntent.action = STOP

            context.startService(serviceIntent)
        }

        private fun prepareVpnService(
            context: Context,
            requestVpnPermission: ActivityResultLauncher<Intent>
        ): Boolean {
            // Prepare vpn service
            val prepareIntent = prepare(context)

            return if (prepareIntent != null) {
                Log.d(TAG, "Don't have VPN permission, asking for it...")

                // Launch the request vpn permission
                requestVpnPermission.launch(prepareIntent)

                false
            } else {
                Log.d(TAG, "App already has VPN permissions.")

                true
            }
        }
    }

    var vpnState: VpnState = VpnState.DISCONNECTED
    var connectedServer: String? = null

    private val binder = LocalBinder()

    private var backend: Backend? = null
    private var wgConfig: Config? = null
    private var tunnel: Tunnel? = null

    private var refreshIntent: PendingIntent? = null

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        Log.d(TAG, "Got 'onStartCommand' INTENT $intent")
        if (intent == null)
            return START_NOT_STICKY

        Log.d(TAG, "Got action: " + intent.action)

        if (START == intent.action) {
            val vpnConnection = intent.getStringExtra(VPN_CONNECTION)
            val serverName = intent.getStringExtra(SERVER_NAME)

            if (vpnConnection != null && serverName != null)
                this.start(vpnConnection, serverName)

            return START_STICKY
        } else if (STOP == intent.action) {
            stop()
            return START_NOT_STICKY
        } else if (REFRESH == intent.action) {
            this.refresh()

            return START_STICKY
        }


        return super.onStartCommand(intent, flags, startId)
    }

    override fun onBind(intent: Intent?): IBinder {
        return this.binder
    }

    private fun start(vpnConnection: String, serverName: String) {
        // Foreground notification is required for the VPN service
        this.setForegroundNotification("Connecting to the VPN...", "", true);

        // Parse the vpn connection profile
        val connectionProfile = WireguardConnection.fromJson(vpnConnection)


        EXECUTOR.execute {
            // Construct the interface
            val wgInterfaceBuilder = Interface.Builder()

            // Add client addresses
            for (address: String in connectionProfile.clientAddresses) {
                wgInterfaceBuilder.addAddress(InetNetwork.parse(address))
            }

            // Set key pair
            wgInterfaceBuilder.setKeyPair(KeyPair(Key.fromBase64(connectionProfile.clientPrivateKey)))

            // Set dns
            wgInterfaceBuilder.addDnsServer(InetNetwork.parse("1.1.1.1").address);
            wgInterfaceBuilder.addDnsServer(InetNetwork.parse("1.0.0.1").address);
            wgInterfaceBuilder.addDnsServer(InetNetwork.parse("2606:4700:4700::1111").address)
            wgInterfaceBuilder.addDnsServer(InetNetwork.parse("2606:4700:4700::1001").address)

            // Exclude requests from the veronymous application
            // Prevents veronymous services from detecting which server a client is connected to
            wgInterfaceBuilder.excludeApplication(this.baseContext.packageName)

            // Excluded routes
            // This is an extra measure for privacy
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU)
                for (hostName in VeronymousConfig.OUT_OF_BAND_HOSTS) {
                    // Resolve the address
                    val address = InetAddress.getByName(hostName)

                    // Assemble the prefix
                    val parcel = Parcel.obtain()
                    parcel.setDataPosition(0)
                    parcel.writeByteArray(address.address)
                    parcel.writeInt(32)
                    parcel.setDataPosition(0)

                    val ipPrefix = IpPrefix.CREATOR.createFromParcel(parcel)

                    wgInterfaceBuilder.excludeRoute(ipPrefix)
                }

            val wgInterface = wgInterfaceBuilder.build()

            // Construct the peer (vpn server)
            val server = Peer.Builder()
                .addAllowedIp(InetNetwork.parse("0.0.0.0/0"))
                .addAllowedIp(InetNetwork.parse("::/0"))
                .setEndpoint(InetEndpoint.parse(connectionProfile.serverEndpoint))
                .setPublicKey(Key.fromBase64(connectionProfile.serverPublicKey))
                .build()

            this.wgConfig = Config.Builder()
                .setInterface(wgInterface)
                .addPeer(server)
                .build()

            if (this.backend == null)
                this.backend = GoBackend(this)

            this.tunnel = VeronymousTunnel()
            this.backend!!.setState(this.tunnel!!, Tunnel.State.UP, this.wgConfig)

            this.vpnState = VpnState.CONNECTED
            this.connectedServer = serverName

            // Schedule reconnection
            this.scheduleConnectionRefresh()

            this.setForegroundNotification(
                "Securely connected to the VPN",
                "You can now browse the internet securely and privately.",
                true
            )
            this.notifyConnected()

            Log.d(TAG, "Wireguard connection has been created!");
        }
    }

    private fun refresh() {
        Log.d(TAG, "Refreshing at: ${Instant.now()}")
        EXECUTOR.execute {
            VeronymousClient.connect(
                this,
                this.connectedServer,
                object : VeronymousTaskListener<String> {
                    override fun onResult(vpnConnection: String) {
                        start(vpnConnection, connectedServer!!)
                    }

                    override fun onError(e: VeronymousClientException?) {
                        Log.e(TAG, "Could not create vpn connection.", e)

                        stop(
                            "VPN connection refresh failure",
                            "An error has occurred when trying to refresh the VPN connection",
                        )
                    }
                })
        }
    }

    private fun scheduleConnectionRefresh() {
        Log.d(TAG, "Scheduling reconnection...")

        // Get seconds until refresh
        val timeToRefresh = VeronymousClient.getTimeToRefresh();

        // Get the actual refresh time
        val timeOfRefresh =
            SystemClock.elapsedRealtime() + TimeUnit.SECONDS.toMillis(timeToRefresh)

        Log.d(TAG, "Scheduling refresh at ${Instant.now().epochSecond + timeToRefresh}")

        val alarmManager = this.getSystemService(Context.ALARM_SERVICE) as AlarmManager

        // Make sure the application has the 'schedule exact alarm permissions'
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S)
            if (!alarmManager.canScheduleExactAlarms()) {
                Log.d(TAG, "Don't have the required permissions to schedule exact alarms.")
                throw IllegalStateException("Cannot schedule exact alarms")
            }


        // Refresh intent
        val serviceIntent = Intent(this, VeronymousVpnService::class.java)
        serviceIntent.action = REFRESH

        this.refreshIntent = PendingIntent.getService(
            this,
            REFRESH_CONNECTION_REQUEST,
            serviceIntent,
            PendingIntent.FLAG_IMMUTABLE
        )

        alarmManager.setExactAndAllowWhileIdle(
            AlarmManager.ELAPSED_REALTIME_WAKEUP,
            timeOfRefresh,
            this.refreshIntent
        )
    }

    private fun stop() {
        this.stop(
            "Disconnected from the VPN",
            "Your device has been disconnected from the VPN"        )
    }

    private fun stop(title: String, message: String) {
        Log.d(TAG, "Disconnecting...")

        if (this.refreshIntent != null) {
            val alarmManager = this.getSystemService(Context.ALARM_SERVICE) as AlarmManager
            alarmManager.cancel(this.refreshIntent)
        }


        if (this.backend != null && this.tunnel != null && this.wgConfig != null) {
            this.setForegroundNotification("Disconnecting from the VPN...", "", true)

            EXECUTOR.execute {
                this.backend!!.setState(this.tunnel!!, Tunnel.State.DOWN, this.wgConfig)


                this.vpnState = VpnState.DISCONNECTED
                this.connectedServer = null
                Log.d(TAG, "Successfully disconnected from the vpn server.")

                this.notifyDisconnected()
                this.setForegroundNotification(
                    title,
                    message,
                    false
                )
            }
        }
    }


    private fun setForegroundNotification(title: String, message: String, ongoing: Boolean) {
        val notificationManager =
            this.getSystemService(NOTIFICATION_SERVICE) as NotificationManager
        notificationManager.createNotificationChannel(
            NotificationChannel(
                NOTIFICATION_ID,
                NOTIFICATION_ID,
                NotificationManager.IMPORTANCE_DEFAULT
            )
        )
        val pendingIntent = PendingIntent.getActivity(
            this,
            VIEW_STATUS_REQUEST,
            Intent(this, MainActivity::class.java),
            PendingIntent.FLAG_MUTABLE
        )
        this.startForeground(
            1,
            Notification.Builder(this, NOTIFICATION_ID)
                .setSmallIcon(R.drawable.veronymous_icon_white_20_20)
                .setContentTitle(title)
                .setContentText(message)
                .setContentIntent(pendingIntent)
                .setOngoing(ongoing)
                .build()
        )

    }

    private fun notifyConnected() {
        val intent = Intent()

        intent.action = CONNECTION_UPDATE

        this.sendBroadcast(intent)
    }

    private fun notifyDisconnected() {
        val intent = Intent()

        intent.action = DISCONNECTION_UPDATE

        this.sendBroadcast(intent)
    }

    inner class LocalBinder : Binder() {
        // Return this instance of LocalService so clients can call public methods.
        fun getService(): VeronymousVpnService = this@VeronymousVpnService
    }

}