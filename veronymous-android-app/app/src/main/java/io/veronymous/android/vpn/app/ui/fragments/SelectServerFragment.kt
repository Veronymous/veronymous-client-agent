package io.veronymous.android.vpn.app.ui.fragments

import android.app.Activity.RESULT_OK
import android.content.Intent
import android.os.Bundle
import android.util.Log
import android.view.View
import android.widget.AdapterView.OnItemClickListener
import android.widget.AdapterView.VISIBLE
import android.widget.Button
import android.widget.ListView
import android.widget.TextView
import androidx.activity.result.ActivityResult
import androidx.activity.result.ActivityResultLauncher
import androidx.activity.result.contract.ActivityResultContracts
import androidx.fragment.app.Fragment
import androidx.fragment.app.commit
import androidx.fragment.app.replace
import io.veronymous.android.veronymous.client.VeronymousClient
import io.veronymous.android.veronymous.client.listener.VeronymousTaskListener
import io.veronymous.android.vpn.app.R
import io.veronymous.android.vpn.app.service.VeronymousVpnService
import io.veronymous.android.vpn.app.ui.fragments.adapters.ServerListAdapter
import io.veronymous.client.exceptions.VeronymousClientException

class SelectServerFragment : Fragment(R.layout.select_server_fragment) {

    companion object {
        private val TAG = SelectServerFragment::class.simpleName
    }

    private var selectedServer: String? = null

    private lateinit var requestVpnPermissionLauncher: ActivityResultLauncher<Intent>

    private lateinit var connectButton: Button

    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
        super.onViewCreated(view, savedInstanceState)

        val title = this.requireActivity().findViewById<TextView>(R.id.main_banner_title);
        title.setText(R.string.select_server_title)


        // Request vpn permission callback
        this.requestVpnPermissionLauncher =
            registerForActivityResult(ActivityResultContracts.StartActivityForResult()) { result: ActivityResult ->
                Log.d(TAG, "Got request permission result. RESULT CODE ${result.resultCode}")

                if (RESULT_OK == result.resultCode) {
                    Log.d(TAG, "VPN permission has been accepted.")

                    this.createVpnConnection()
                } else {
                    Log.d(TAG, "VPN permission request has failed.")

                }
            }


        this.connectButton = view.findViewById(R.id.vpn_connect_button);
        this.connectButton.setOnClickListener {
            this.createVpnConnection()
        }

        val serversListView = view.findViewById<ListView>(R.id.servers_list_view)
        this.loadServersListView(serversListView)
    }

    override fun onResume() {
        Log.d(TAG, "On resume");
        super.onResume()
    }

    private fun createVpnConnection() {
        Log.d(TAG, "Creating VPN connection...");

        if (this.selectedServer != null && this.context != null) {
            val arguments = Bundle();
            arguments.putString(ConnectingFragment.SERVER_NAME, this.selectedServer)

            this.parentFragmentManager.commit {
                replace<ConnectingFragment>(R.id.main_activity_fragment_container, args = arguments)
                setReorderingAllowed(true)
            }
        }
    }

    // TODO: Add loader
    private fun loadServersListView(listView: ListView) {
        // Get the servers list
        VeronymousClient.getServers(this.context, object : VeronymousTaskListener<Array<String>> {
            override fun onResult(servers: Array<String>) {
                Log.d(TAG, "Got list of servers.")
                Log.d(TAG, "Populating servers adapter.")

                activity?.runOnUiThread {
                    populateServersListView(listView, servers)
                }
            }

            override fun onError(e: VeronymousClientException?) {
                Log.d(TAG, "Could not get servers.", e)

                // TODO: Handle error
            }

        })
    }

    private fun populateServersListView(listView: ListView, servers: Array<String>) {
        val listViewAdapter = this.activity?.let { ServerListAdapter(it, servers) }
        listView.adapter = listViewAdapter

        listView.onItemClickListener =
            OnItemClickListener { _, _, position, _ ->
                listViewAdapter?.setSelected(position)
                listViewAdapter?.notifyDataSetChanged()


                if (listViewAdapter != null) {
                    val selectedServer = listViewAdapter.getItem(position)

                    this.selectServer(selectedServer);
                }
                Log.d(TAG, "Selected server: $selectedServer")

            }
    }

    private fun selectServer(server: String) {
        if (!this.isServerSelected())
            this.activateConnectButton();

        this.selectedServer = server;
    }

    private fun activateConnectButton() {
        this.connectButton.isEnabled = true;

        this.context?.let {
            this.connectButton.visibility = VISIBLE
        }
    }

    private fun isServerSelected(): Boolean {
        return this.selectedServer != null;
    }

}