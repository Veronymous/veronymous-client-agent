package io.veronymous.android.vpn.app.ui.dialog

import android.app.AlertDialog
import android.app.Dialog
import android.content.Intent
import android.os.Build
import android.os.Bundle
import android.provider.Settings
import androidx.annotation.RequiresApi
import androidx.fragment.app.DialogFragment
import io.veronymous.android.vpn.app.R

class RequestAlarmDialog : DialogFragment() {
    @RequiresApi(Build.VERSION_CODES.S)
    override fun onCreateDialog(savedInstanceState: Bundle?): Dialog {
        val builder = AlertDialog.Builder(requireActivity())

        builder.setTitle(this.getString(R.string.enable_exact_alarm_title))
            .setMessage(this.getString(R.string.enable_exact_alarm_message))
            .setPositiveButton(R.string.enable) { dialog, _ ->
                run {
                    this.startActivity(Intent(Settings.ACTION_REQUEST_SCHEDULE_EXACT_ALARM))
                    dialog.dismiss()
                }
            }
            .setNegativeButton(android.R.string.cancel) { dialog, _ ->
                run {
                    dialog.dismiss()
                }
            }

        return builder.create()
    }
}