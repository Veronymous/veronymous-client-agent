package io.veronymous.android.vpn.app.ui.fragments

import android.os.Bundle
import android.util.Log
import android.view.View
import android.widget.Button
import android.widget.EditText
import android.widget.TextView
import androidx.fragment.app.Fragment
import androidx.fragment.app.FragmentManager
import androidx.fragment.app.commit
import androidx.fragment.app.replace
import io.veronymous.android.veronymous.client.VeronymousClient
import io.veronymous.android.veronymous.client.listener.VeronymousTaskListener
import io.veronymous.android.veronymous.client.status.AuthStatus
import io.veronymous.android.vpn.app.R
import io.veronymous.client.exceptions.VeronymousClientException

class AuthFragment : Fragment(R.layout.login_fragment) {

    companion object {
        private val TAG = AuthFragment::class.simpleName;
    }


    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
        val title = this.requireActivity().findViewById<TextView>(R.id.main_banner_title);
        title.setText(R.string.authenticate_title)

        val emailInput = view.findViewById<EditText>(R.id.auth_email_input);
        val passwordInput = view.findViewById<EditText>(R.id.auth_password_input);

        val authFailedMessage = view.findViewById<TextView>(R.id.auth_error_message)

        val authButton = view.findViewById<Button>(R.id.auth_button);

        authButton.setOnClickListener { authenticate(emailInput, passwordInput, authFailedMessage) }

        super.onViewCreated(view, savedInstanceState)
    }

    private fun authenticate(
        emailInput: EditText,
        passwordInput: EditText,
        authErrorMessage: TextView
    ) {
        Log.d(TAG, "Authenticating...");

        // Reset teh auth error message view
        authErrorMessage.visibility = View.VISIBLE

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
                        AuthStatus.SUBSCRIPTION_REQUIRED -> goToSubscribeView()
                        AuthStatus.AUTHENTICATION_REQUIRED -> handleAuthFailed(
                            emailInput,
                            passwordInput,
                            authErrorMessage
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

    private fun goToSubscribeView() {
        Log.d(TAG, "Going to the subscription view.")
    }

    private fun handleAuthFailed(
        emailInput: EditText,
        passwordInput: EditText,
        authErrorMessage: TextView
    ) {
        emailInput.setText("")
        passwordInput.setText("")
        authErrorMessage.visibility = View.VISIBLE
    }
}