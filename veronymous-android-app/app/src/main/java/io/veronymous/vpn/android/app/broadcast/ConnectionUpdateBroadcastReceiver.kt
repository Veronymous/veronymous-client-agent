package io.veronymous.vpn.android.app.broadcast

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.IntentFilter
import android.util.Log
import io.veronymous.vpn.android.app.service.VeronymousVpnService
import io.veronymous.vpn.android.app.ui.activities.MainActivity

class ConnectionUpdateBroadcastReceiver(private val activity: MainActivity) : BroadcastReceiver() {
    companion object {
        private val TAG = ConnectionUpdateBroadcastReceiver::class.simpleName

        fun intentFilter(): IntentFilter {
            val broadcastFilter = IntentFilter()
            broadcastFilter.addAction(VeronymousVpnService.DISCONNECTION_UPDATE)
            broadcastFilter.addAction(VeronymousVpnService.CONNECTION_UPDATE)

            return broadcastFilter
        }
    }

    override fun onReceive(context: Context?, intent: Intent?) {
        if (intent != null) {
            Log.d(TAG, "Got broadcast ${intent.action}")

            if (VeronymousVpnService.CONNECTION_UPDATE == intent.action)
                this.activity.notifyConnectionUpdate()
            else if (VeronymousVpnService.DISCONNECTION_UPDATE == intent.action)
                this.activity.notifyDisconnected()

        }
    }
}