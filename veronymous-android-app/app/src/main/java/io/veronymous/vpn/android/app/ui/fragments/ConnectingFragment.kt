package io.veronymous.vpn.android.app.ui.fragments

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
import io.veronymous.vpn.android.app.R
import io.veronymous.vpn.android.app.service.VeronymousVpnService
import io.veronymous.vpn.android.app.service.listener.ConnectionResultListener
import io.veronymous.vpn.android.app.ui.config.ServerConfigs
import io.veronymous.vpn.android.app.ui.dialog.ActionDialog

class ConnectingFragment : AppFragment(R.layout.connecting_layout) {

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
    override fun showInfoPrompt() {
        // Do nothing
    }

    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
        super.onViewCreated(view, savedInstanceState)

        this.requireActivity().actionBar?.setTitle(R.string.connecting_title)

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
                this.selectedServer!!,
                object: ConnectionResultListener {
                    override fun onSuccess() {
                        Log.d(TAG, "Successfully created VPN connection.")
                    }

                    override fun onFailure(e: Exception?) {
                        Log.d(TAG, "Could not establish VPN connection.", e)

                        ActionDialog(
                            getString(R.string.could_not_connect_title),
                            getString(R.string.could_not_connect_message)
                        ) {
                            goToServersView()
                        }.show(parentFragmentManager, "CONNECTION_ERROR")

                    }

                }
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