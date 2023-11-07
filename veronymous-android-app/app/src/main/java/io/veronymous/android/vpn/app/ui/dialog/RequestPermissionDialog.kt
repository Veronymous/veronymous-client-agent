package io.veronymous.android.vpn.app.ui.dialog

import android.app.AlertDialog
import android.app.Dialog
import android.os.Bundle
import androidx.activity.result.ActivityResultLauncher
import androidx.fragment.app.DialogFragment
import io.veronymous.android.vpn.app.R

class RequestPermissionDialog(
    private val title: String,
    private val message: String,
    private val permission: String,
    private val launcher: ActivityResultLauncher<String>
) : DialogFragment() {

    override fun onCreateDialog(savedInstanceState: Bundle?): Dialog {
        val builder = AlertDialog.Builder(requireActivity())
        builder.setTitle(title)
        builder.setMessage(message)

        builder.setPositiveButton(R.string.enable) { dialog, _ ->
            run {
                this.launcher.launch(permission)
                dialog.dismiss()
            }
        }
        builder.setNegativeButton(android.R.string.cancel) { dialog, _ ->
            run {
                dialog.dismiss()
            }
        }

        return builder.create()
    }
}