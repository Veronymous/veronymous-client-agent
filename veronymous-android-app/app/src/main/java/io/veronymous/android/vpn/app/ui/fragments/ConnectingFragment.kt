package io.veronymous.android.vpn.app.ui.fragments

import android.app.Activity
import android.content.Intent
import android.os.Bundle
import android.util.Log
import android.view.View
import android.widget.TextView
import androidx.activity.result.ActivityResult
import androidx.activity.result.ActivityResultLauncher
import androidx.activity.result.contract.ActivityResultContracts
import androidx.fragment.app.Fragment
import androidx.fragment.app.commit
import androidx.fragment.app.replace
import io.veronymous.android.vpn.app.R
import io.veronymous.android.vpn.app.service.VeronymousVpnService
import io.veronymous.android.vpn.app.ui.config.ServerConfigs

class ConnectingFragment : Fragment(R.layout.connecting_layout) {

    companion object {
        const val SERVER_NAME = "server_name";

        private val TAG = ConnectingFragment::class.simpleName;

        private fun getServerTitle(serverName: String): String {
            val serverConfig = ServerConfigs.SERVERS[serverName];

            return serverConfig?.displayName ?: "VPN"
        }
    }

    private lateinit var requestVpnPermissionLauncher: ActivityResultLauncher<Intent>

    private var selectedServer: String? = null;

    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
        super.onViewCreated(view, savedInstanceState)

        val title = this.requireActivity().findViewById<TextView>(R.id.main_banner_title);
        title.setText(R.string.connecting_title)

        // Request vpn permission callback
        this.requestVpnPermissionLauncher =
            registerForActivityResult(ActivityResultContracts.StartActivityForResult()) { result: ActivityResult ->
                Log.d(TAG, "Got request permission result. RESULT CODE ${result.resultCode}")

                if (Activity.RESULT_OK == result.resultCode) {
                    Log.d(TAG, "VPN permission has been accepted.")

                    this.createVpnConnection()
                } else {
                    Log.d(TAG, "VPN permission request has failed.")

                    this.goToServersView()
                }
            }

        val connectingTextView = view.findViewById<TextView>(R.id.connecting_text_view);

        this.selectedServer = this.arguments?.getString(SERVER_NAME)

        val serverTitle = this.selectedServer?.let { getServerTitle(it) };
        connectingTextView.text = resources.getString(R.string.connecting_to_location, serverTitle)

        this.createVpnConnection()
    }

    private fun createVpnConnection() {
        if (this.selectedServer != null)
            VeronymousVpnService.connect(
                this.requireActivity(),
                this.requestVpnPermissionLauncher,
                this.selectedServer!!
            )
    }

    private fun goToServersView() {
        Log.d(TAG, "Navigating to the servers view.")

        this.parentFragmentManager.commit {
            replace<SelectServerFragment>(R.id.main_activity_fragment_container)
            setReorderingAllowed(true)
        }
    }

}