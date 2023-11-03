package io.veronymous.android.vpn.app.ui.fragments

import android.os.Bundle
import android.util.Log
import android.view.View
import android.widget.Button
import android.widget.ImageView
import android.widget.TextView
import androidx.fragment.app.Fragment
import androidx.fragment.app.commit
import androidx.fragment.app.replace
import io.veronymous.android.vpn.app.R
import io.veronymous.android.vpn.app.service.VeronymousVpnService
import io.veronymous.android.vpn.app.ui.config.ServerConfigs

class ConnectionFragment : Fragment(R.layout.connection_fragment) {

    companion object {
        private val TAG = ConnectionFragment::class.simpleName

        const val SERVER_NAME = "server_name"

        private fun loadConnectedServerView(
            selectedServer: String?,
            serverNameView: TextView,
            serverFlagView: ImageView
        ) {
            if (selectedServer != null) {
                val serverConfig = ServerConfigs.SERVERS[selectedServer]

                if (serverConfig != null) {

                    serverNameView.text = serverConfig.displayName
                    serverFlagView.setImageResource(serverConfig.flag)

                }
            }
        }
    }

    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
        super.onViewCreated(view, savedInstanceState)

        val title = this.requireActivity().findViewById<TextView>(R.id.main_banner_title);
        title.setText(R.string.connected_title)

        val serverNameView = view.findViewById<TextView>(R.id.connected_server_text_view)
        val serverFlagView = view.findViewById<ImageView>(R.id.connected_server_flag_view);

        val selectedServer = this.arguments?.getString(ConnectingFragment.SERVER_NAME)

        loadConnectedServerView(selectedServer, serverNameView, serverFlagView)

        val disconnectButton = view.findViewById<Button>(R.id.disconnect_button)
        disconnectButton.setOnClickListener {
            this.disconnectVpn()
        }
    }

    private fun disconnectVpn() {
        Log.d(TAG, "Disconnecting VPN...")

        this.parentFragmentManager.commit {
            replace<DisconnectingFragment>(R.id.main_activity_fragment_container)
            setReorderingAllowed(false)
        }
    }
}