package io.veronymous.vpn.android.app.ui.fragments

import androidx.fragment.app.Fragment

abstract class AppFragment(contentLayoutId: Int) : Fragment(contentLayoutId) {

    abstract fun showInfoPrompt()
}