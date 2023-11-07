package io.veronymous.android.vpn.app.ui.dialog

import android.app.AlertDialog
import android.app.Dialog
import android.os.Bundle
import androidx.fragment.app.DialogFragment
import io.veronymous.android.vpn.app.R

class InfoDialog(private val title: String, private val message: String) : DialogFragment() {

    override fun onCreateDialog(savedInstanceState: Bundle?): Dialog {
        val builder = AlertDialog.Builder(requireActivity())

        builder
            .setTitle(title)
            .setMessage(message)
            .setPositiveButton(R.string.OK) { dialog, _ ->
                run {
                    dialog.dismiss()
                }
            }

        return builder.create()
    }
}