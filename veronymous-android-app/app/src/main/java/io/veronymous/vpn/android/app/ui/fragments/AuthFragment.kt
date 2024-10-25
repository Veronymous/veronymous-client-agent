package io.veronymous.vpn.android.app.ui.fragments

import android.content.Intent
import android.graphics.Color
import android.net.Uri
import android.os.Bundle
import android.text.Spannable
import android.text.SpannableStringBuilder
import android.text.style.ForegroundColorSpan
import android.util.Log
import android.view.View
import android.widget.Button
import android.widget.EditText
import android.widget.TextView
import androidx.fragment.app.Fragment
import androidx.fragment.app.commit
import androidx.fragment.app.replace
import io.veronymous.android.veronymous.client.VeronymousClient
import io.veronymous.android.veronymous.client.listener.VeronymousTaskListener
import io.veronymous.android.veronymous.client.status.AuthStatus
import io.veronymous.vpn.android.app.ui.dialog.InfoDialog
import io.veronymous.client.exceptions.VeronymousClientException
import io.veronymous.vpn.android.app.R

class AuthFragment : AppFragment(R.layout.login_fragment) {

    companion object {
        private val TAG = AuthFragment::class.simpleName;

        private const val VERONYMOUS_IO = "veronymous.io"


        private const val SUBSCRIPTIONS_URL = "https://veronymous.io/portal/subscriptions"
    }

    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
        val activity = this.requireActivity()

        activity.actionBar?.setTitle(R.string.get_zk_creds_title)

        val emailInput = view.findViewById<EditText>(R.id.auth_email_input);
        val passwordInput = view.findViewById<EditText>(R.id.auth_password_input);

        val registerMessage = view.findViewById<TextView>(R.id.auth_register_message)
        registerMessage.setOnClickListener {
            this.goToSubscribe()
        }

        val authFailedMessage = view.findViewById<TextView>(R.id.auth_error_message)
        authFailedMessage.setOnClickListener {
            this.goToSubscribe()
        }

        val authButton = view.findViewById<Button>(R.id.auth_button);

        authButton.setOnClickListener {
            authenticate(
                emailInput,
                passwordInput,
                authFailedMessage,
                registerMessage
            )
        }

        super.onViewCreated(view, savedInstanceState)
    }

    private fun authenticate(
        emailInput: EditText,
        passwordInput: EditText,
        authErrorMessage: TextView,
        registerMessage: TextView
    ) {
        Log.d(TAG, "Authenticating...");

        // Reset the auth error message view
        authErrorMessage.visibility = View.INVISIBLE
//        registerMessage.visibility = View.VISIBLE

        val email = emailInput.text.toString()
        val password = passwordInput.text.toString()

        VeronymousClient.authenticate(
            this.context,
            email,
            password,
            object : VeronymousTaskListener<AuthStatus> {
                override fun onResult(status: AuthStatus?) {
                    Log.d(TAG, "Authentication result: " + status.toString());

                    when (status) {
                        AuthStatus.AUTHENTICATED -> goToServersView()
                        AuthStatus.SUBSCRIPTION_REQUIRED -> handleSubscriptionRequired(
                            emailInput,
                            passwordInput,
                            authErrorMessage,
                            registerMessage
                        )

                        AuthStatus.AUTHENTICATION_REQUIRED -> handleAuthFailed(
                            emailInput,
                            passwordInput,
                            authErrorMessage,
                            registerMessage
                        )

                        else -> {
                            Log.d(TAG, "Got unsupported AuthStatus " + status.toString())
                        }
                    }
                }

                override fun onError(e: VeronymousClientException?) {
                    Log.d(TAG, "Could not authenticate", e)

                    // TODO: Handle error
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

    private fun handleSubscriptionRequired(
        emailInput: EditText,
        passwordInput: EditText,
        authErrorMessage: TextView,
        registerMessage: TextView
    ) {
        this.requireActivity().runOnUiThread {
            emailInput.setText("")
            passwordInput.setText("")


            registerMessage.visibility = View.INVISIBLE

            val message = getString(R.string.subscription_required_message)
            val span =
                SpannableStringBuilder(message)

            val veronymousIndex = message.indexOf(VERONYMOUS_IO)

            span.setSpan(
                ForegroundColorSpan(Color.BLUE),
                veronymousIndex,
                veronymousIndex + VERONYMOUS_IO.length,
                Spannable.SPAN_EXCLUSIVE_EXCLUSIVE
            )

            authErrorMessage.text = span

            authErrorMessage.visibility = View.VISIBLE
        }
    }

    private fun handleAuthFailed(
        emailInput: EditText,
        passwordInput: EditText,
        authErrorMessage: TextView,
        registerMessage: TextView
    ) {
        this.requireActivity().runOnUiThread {
            emailInput.setText("")
            passwordInput.setText("")

            registerMessage.visibility = View.INVISIBLE

            val message = getString(R.string.invalid_email_or_password_message)
            val span =
                SpannableStringBuilder(message)

            val veronymousIndex = message.indexOf(VERONYMOUS_IO)

            span.setSpan(
                ForegroundColorSpan(Color.BLUE),
                veronymousIndex,
                veronymousIndex + VERONYMOUS_IO.length,
                Spannable.SPAN_EXCLUSIVE_EXCLUSIVE
            )

            authErrorMessage.text = span
            authErrorMessage.visibility = View.VISIBLE
        }
    }

    private fun goToSubscribe() {
        val intent = Intent(Intent.ACTION_VIEW, Uri.parse(SUBSCRIPTIONS_URL))
        startActivity(intent)
    }

    override fun showInfoPrompt() {
        InfoDialog(
            this.getString(R.string.get_zk_creds_full_title),
            this.getString(R.string.get_zk_creds_info)
        ).show(this.parentFragmentManager, "AUTH_INFO")
    }
}