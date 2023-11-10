package io.veronymous.android.vpn.app.ui.fragments

import android.Manifest
import android.app.Activity.RESULT_OK
import android.app.AlarmManager
import android.content.Context
import android.content.Intent
import android.content.pm.PackageManager
import android.os.Build
import android.os.Bundle
import android.provider.Settings
import android.util.Log
import android.view.View
import android.widget.AdapterView.OnItemClickListener
import android.widget.AdapterView.VISIBLE
import android.widget.Button
import android.widget.ImageButton
import android.widget.ListView
import android.widget.TextView
import androidx.activity.result.ActivityResult
import androidx.activity.result.ActivityResultLauncher
import androidx.activity.result.contract.ActivityResultContracts
import androidx.annotation.RequiresApi
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import androidx.fragment.app.Fragment
import androidx.fragment.app.commit
import androidx.fragment.app.replace
import io.veronymous.android.veronymous.client.VeronymousClient
import io.veronymous.android.veronymous.client.listener.VeronymousTaskListener
import io.veronymous.android.vpn.app.R
import io.veronymous.android.vpn.app.service.VeronymousVpnService
import io.veronymous.android.vpn.app.ui.dialog.InfoDialog
import io.veronymous.android.vpn.app.ui.dialog.RequestAlarmDialog
import io.veronymous.android.vpn.app.ui.dialog.RequestPermissionDialog
import io.veronymous.android.vpn.app.ui.fragments.adapters.ServerListAdapter
import io.veronymous.client.exceptions.VeronymousClientException
import java.lang.UnsupportedOperationException

class SelectServerFragment : Fragment(R.layout.select_server_fragment) {

    companion object {
        private val TAG = SelectServerFragment::class.simpleName
    }

    private var selectedServer: String? = null

    private lateinit var requestPermissionLauncher: ActivityResultLauncher<String>

    private lateinit var requestVpnPermissionLauncher: ActivityResultLauncher<Intent>

    private lateinit var connectButton: Button

    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
        super.onViewCreated(view, savedInstanceState)

        val activity = requireActivity()

        val title = activity.findViewById<TextView>(R.id.main_banner_title);
        title.setText(R.string.select_server_title)

        val infoButton = activity.findViewById<ImageButton>(R.id.info_button)
        infoButton.setOnClickListener { showInfoButton() }
        infoButton.visibility = View.VISIBLE

        // Request permissions
        this.requestPermissionLauncher = registerForActivityResult(
            ActivityResultContracts.RequestPermission()
        ) { isGranted: Boolean ->
            Log.d(TAG, "Permission granted: $isGranted")
        }


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
        // TODO: Check permissions
        Log.d(TAG, "Creating VPN connection...");

        if (this.selectedServer != null && this.context != null && this.hasRequiredPermissions()) {
            val arguments = Bundle();
            arguments.putString(ConnectingFragment.SERVER_NAME, this.selectedServer)

            this.parentFragmentManager.commit {
                replace<ConnectingFragment>(R.id.main_activity_fragment_container, args = arguments)
                setReorderingAllowed(true)
            }
        }
    }

    private fun hasRequiredPermissions(): Boolean {
        val alarmManager =
            this.requireActivity().getSystemService(Context.ALARM_SERVICE) as AlarmManager

        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S && !alarmManager.canScheduleExactAlarms()) {
            Log.d(TAG, "Missing 'canScheduleExecuteAlarms' permission.")
            this.requestScheduleExactAlarmPermission()

            return false
        } else if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU && ContextCompat.checkSelfPermission(
                this.requireContext(),
                Manifest.permission.POST_NOTIFICATIONS
            ) != PackageManager.PERMISSION_GRANTED
        ) {
            Log.d(TAG, "Missing 'POST_NOTIFICATION' permission.")
            this.requestNotificationsPermission()

            // TODO: Request permission for different sdk versions?

            return false
        }

        return true
    }

    private fun requestScheduleExactAlarmPermission() {
        RequestAlarmDialog().show(this.parentFragmentManager, "ENABLE_SCHEDULE_EXACT_ALARM")
    }

    @RequiresApi(Build.VERSION_CODES.TIRAMISU)
    private fun requestNotificationsPermission() {
        // Notifications permission is required for
        RequestPermissionDialog(
            this.getString(R.string.enable_notifications_title),
            this.getString(R.string.enable_exact_alarm_message),
            Manifest.permission.POST_NOTIFICATIONS,
            this.requestPermissionLauncher
        ).show(this.parentFragmentManager, "REQUEST_NOTIFICATIONS_PERMISSION")

    }

    // TODO: Add loader
    private fun loadServersListView(listView: ListView) {
        // Get the servers list
        VeronymousClient.getServers(
            this.context,
            object : VeronymousTaskListener<Array<String>> {
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

    private fun showInfoButton() {
        InfoDialog(
            this.getString(R.string.select_server_title),
            this.getString(R.string.select_server_info)
        ).show(this.parentFragmentManager, "AUTH_INFO")
    }
}