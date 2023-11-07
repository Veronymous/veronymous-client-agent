package io.veronymous.android.vpn.app.ui.fragments

import android.os.Bundle
import android.util.Log
import android.view.View
import android.widget.ImageButton
import android.widget.TextView
import androidx.fragment.app.Fragment
import io.veronymous.android.vpn.app.R
import io.veronymous.android.vpn.app.service.VeronymousVpnService

class DisconnectingFragment : Fragment(R.layout.disconnecting_layout) {

    companion object {
        private val TAG = DisconnectingFragment::class.simpleName
    }

    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
        super.onViewCreated(view, savedInstanceState)

        val activity = requireActivity()

        val title = activity.findViewById<TextView>(R.id.main_banner_title);
        title.setText(R.string.disconnecting_title)

        val infoButton = activity.findViewById<ImageButton>(R.id.info_button)
        infoButton.visibility = View.INVISIBLE

        this.disconnectVpn()
    }


    private fun disconnectVpn() {
        Log.d(TAG, "Disconnecting VPN...")

        VeronymousVpnService.disconnect(this.requireContext())
    }
}