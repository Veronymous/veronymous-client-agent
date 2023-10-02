package io.veronymous.client.jni;

import io.veronymous.client.exceptions.IllegalStateException;

public class AuthenticateResult {

    private final String clientState;
    private final boolean subscriptionRequired;
    private final boolean hasError;
    private final String error;

    public AuthenticateResult(String clientState, boolean subscriptionRequired, boolean hasError, String error) {
        this.clientState = clientState;
        this.subscriptionRequired = subscriptionRequired;
        this.hasError = hasError;
        this.error = error;
    }

    public String getClientState() throws IllegalStateException {
        if (this.hasError)
            throw new IllegalStateException("Has error");

        if (this.subscriptionRequired)
            throw new IllegalStateException("Subscription is required");

        return clientState;
    }

    public boolean isSubscriptionRequired() {
        return subscriptionRequired;
    }

    public boolean hasError() {
        return hasError;
    }

    public String getError() throws IllegalStateException {
        if (!this.hasError)
            throw new IllegalStateException("Does not have an error.");
        return error;
    }
}
