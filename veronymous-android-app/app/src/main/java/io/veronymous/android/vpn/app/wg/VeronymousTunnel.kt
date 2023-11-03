package io.veronymous.android.vpn.app.wg

import android.util.Log
import com.wireguard.android.backend.Tunnel

class VeronymousTunnel : Tunnel {

    companion object {
        private val TAG = VeronymousTunnel::class.simpleName

        private const val NAME = "veron0";
    }

    override fun getName(): String {
        return NAME
    }

    override fun onStateChange(newState: Tunnel.State) {
        Log.d(TAG, "Got new state $newState")
        // TODO: Handle this
    }
}