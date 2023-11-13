package io.veronymous.android.vpn.app.ui.dialog

import android.app.AlertDialog
import android.app.Dialog
import android.os.Bundle
import androidx.fragment.app.DialogFragment
import io.veronymous.android.vpn.app.R

// Performs action after user acknowledges the message
class ActionDialog(
    private val title: String,
    private val message: String,
    private val action: Runnable
) : DialogFragment() {

    override fun onCreateDialog(savedInstanceState: Bundle?): Dialog {
        val builder = AlertDialog.Builder(requireActivity())

        builder
            .setTitle(title)
            .setMessage(message)
            .setPositiveButton(R.string.OK) { dialog, _ ->
                run {
                    this.action.run()
                    dialog.dismiss()
                }
            }

        return builder.create()
    }

}