package io.veronymous.vpn.android.app.ui.activities

import android.content.ComponentName
import android.content.Context
import android.content.Intent
import android.content.ServiceConnection
import android.net.Uri
import android.os.Bundle
import android.os.IBinder
import android.util.Log
import android.view.Menu
import android.view.MenuItem
import androidx.core.content.ContextCompat
import androidx.fragment.app.Fragment
import androidx.fragment.app.FragmentActivity
import androidx.fragment.app.FragmentManager
import androidx.fragment.app.commit
import androidx.fragment.app.replace
import io.veronymous.android.veronymous.client.VeronymousClient
import io.veronymous.android.veronymous.client.listener.VeronymousTaskListener
import io.veronymous.android.veronymous.client.status.AuthStatus
import io.veronymous.vpn.android.app.broadcast.ConnectionUpdateBroadcastReceiver
import io.veronymous.vpn.android.app.service.VeronymousVpnService
import io.veronymous.vpn.android.app.state.VpnState
import io.veronymous.vpn.android.app.ui.fragments.AuthFragment
import io.veronymous.vpn.android.app.ui.fragments.ConnectionFragment
import io.veronymous.vpn.android.app.ui.fragments.SelectServerFragment
import io.veronymous.client.exceptions.VeronymousClientException
import io.veronymous.vpn.android.app.R
import io.veronymous.vpn.android.app.ui.fragments.AppFragment


class MainActivity : FragmentActivity(R.layout.activity_main) {

    companion object {
        private val TAG = MainActivity::class.simpleName

        private val PRIVACY_POLICY_URL = Uri.parse("https://veronymous.io/privacyPolicy")
    }

    private lateinit var broadcastReceiver: ConnectionUpdateBroadcastReceiver;
    private var broadcastReceiverRegistered = false;

    private lateinit var vpnService: VeronymousVpnService;
    private var vpnServiceBound = false;

    private val vpnServiceConnection = object : ServiceConnection {
        override fun onServiceConnected(name: ComponentName?, service: IBinder?) {
            val binder = service as VeronymousVpnService.LocalBinder
            vpnService = binder.getService()
            vpnServiceBound = true

            updateView()
        }

        override fun onServiceDisconnected(name: ComponentName?) {
            vpnServiceBound = false
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        Log.d(TAG, "VeronymousVPN 'OnCreate'.");

        this.actionBar?.setLogo(R.drawable.logo_without_name_white_65_65)
        this.actionBar?.setBackgroundDrawable(this.getDrawable(R.drawable.veron_solid))
        this.actionBar?.elevation = 0F;

        this.actionBar?.setDisplayShowHomeEnabled(true);
        this.actionBar?.setDisplayUseLogoEnabled(true);

        this.broadcastReceiver = ConnectionUpdateBroadcastReceiver(this)

        ContextCompat.registerReceiver(
            this,
            broadcastReceiver,
            ConnectionUpdateBroadcastReceiver.intentFilter(),
            ContextCompat.RECEIVER_NOT_EXPORTED
        )
        this.broadcastReceiverRegistered = true;
    }

    override fun onCreateOptionsMenu(menu: Menu?): Boolean {
        this.menuInflater.inflate(R.menu.menu_options, menu)
        return true
    }

    override fun onOptionsItemSelected(item: MenuItem): Boolean {
        if (R.id.option_about == item.itemId) {
            this.showFragmentInfo()
            return true
        } else if (R.id.option_privacy_policy == item.itemId) {
            this.showPrivacyPolicy()
            return true
        }

        return super.onOptionsItemSelected(item)
    }

    override fun onStart() {
        super.onStart()

        // Bind vpnService
        Intent(this, VeronymousVpnService::class.java).also { intent ->
            this.bindService(intent, this.vpnServiceConnection, Context.BIND_AUTO_CREATE)
        }
    }


    override fun onStop() {
        super.onStop()

        this.unbindService(this.vpnServiceConnection)
        this.vpnServiceBound = false
    }

    override fun onResume() {
        super.onResume()

        if (!this.broadcastReceiverRegistered) {
            ContextCompat.registerReceiver(
                this,
                this.broadcastReceiver,
                ConnectionUpdateBroadcastReceiver.intentFilter(),
                ContextCompat.RECEIVER_NOT_EXPORTED
            )
            this.broadcastReceiverRegistered = true;
        }
    }

    override fun onPause() {
        super.onPause()

        if (this.broadcastReceiverRegistered) {
            this.unregisterReceiver(this.broadcastReceiver)
            this.broadcastReceiverRegistered = false
        }
    }

    override fun onDestroy() {
        super.onDestroy()

        if (this.broadcastReceiverRegistered) {
            this.unregisterReceiver(this.broadcastReceiver)
            this.broadcastReceiverRegistered = false
        }
    }

    private fun showPrivacyPolicy() {
        val intent = Intent(Intent.ACTION_VIEW, PRIVACY_POLICY_URL)
        startActivity(intent)
    }

    private fun showFragmentInfo() {
        val currentFragment =
            this.supportFragmentManager.findFragmentById(R.id.main_activity_fragment_container)

        if (currentFragment == null || currentFragment !is AppFragment)
            return

        currentFragment.showInfoPrompt()
    }

    private fun updateView() {
        Log.d(TAG, "Updating view...")

        if (this.vpnServiceBound && this.vpnService.vpnState == VpnState.CONNECTED) {
            // Vpn service is running and is already connected to a vpn server
            this.goToConnectionView()
        } else {
            // TODO: Add some sort of loader

            // Refresh the auth token
            VeronymousClient.refreshAuthToken(
                this,
                object : VeronymousTaskListener<AuthStatus> {
                    override fun onResult(status: AuthStatus?) {
                        Log.d(TAG, "Authentication status: " + status.toString())

                        when (status) {
                            AuthStatus.AUTHENTICATED -> goToServersView()
                            AuthStatus.AUTHENTICATION_REQUIRED -> goToAuthView()
                            AuthStatus.SUBSCRIPTION_REQUIRED -> goToSubscribeView()
                            else -> {
                                Log.d(TAG, "Got unsupported AuthStatus " + status.toString())
                            }
                        }
                    }

                    override fun onError(e: VeronymousClientException?) {
                        Log.d(TAG, "Could not refresh auth token.", e)

                        // TODO: Handle error
                    }
                });
        }
    }

    private fun goToConnectionView() {
        Log.d(TAG, "Navigating to the connection view...")

        val arguments = Bundle()
        arguments.putString(ConnectionFragment.SERVER_NAME, this.vpnService.connectedServer)

        this.supportFragmentManager.commit {
            replace<ConnectionFragment>(R.id.main_activity_fragment_container, args = arguments)
            setReorderingAllowed(true)
        }
    }

    private fun goToServersView() {
        Log.d(TAG, "Navigating to servers view...");

        this.supportFragmentManager.commit {
            replace<SelectServerFragment>(R.id.main_activity_fragment_container)
            setReorderingAllowed(true)
        }
    }

    private fun goToAuthView() {
        Log.d(TAG, "Navigating to authentication view...");


        val fragmentManager: FragmentManager = this.supportFragmentManager
        fragmentManager.commit {
            replace<AuthFragment>(R.id.main_activity_fragment_container)
            setReorderingAllowed(true)
        }

    }

    private fun goToSubscribeView() {
        Log.d(TAG, "Navigating to subscription view...");
    }

    fun notifyConnectionUpdate() {
        Log.d(TAG, "VPN connection established")

        Log.d(TAG, "Is VpnService bound: ${this.vpnServiceBound}")

        this.updateView()
    }

    fun notifyDisconnected() {
        Log.d(TAG, "VPN connection has been terminated.")

        this.updateView()
    }
}