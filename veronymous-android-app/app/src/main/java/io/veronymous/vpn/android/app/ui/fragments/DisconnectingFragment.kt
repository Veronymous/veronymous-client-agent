package io.veronymous.vpn.android.app.ui.fragments

import android.os.Bundle
import android.util.Log
import android.view.View
import android.widget.ImageButton
import android.widget.TextView
import androidx.fragment.app.Fragment
import io.veronymous.vpn.android.app.R
import io.veronymous.vpn.android.app.service.VeronymousVpnService

class DisconnectingFragment : AppFragment(R.layout.disconnecting_layout) {

    companion object {
        private val TAG = DisconnectingFragment::class.simpleName
    }

    override fun showInfoPrompt() {
        // Do nothing
    }

    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
        super.onViewCreated(view, savedInstanceState)

        val activity = requireActivity()

        activity.actionBar?.setTitle(R.string.disconnecting_title)

        this.disconnectVpn()
    }


    private fun disconnectVpn() {
        Log.d(TAG, "Disconnecting VPN...")

        VeronymousVpnService.disconnect(this.requireContext())
    }
}