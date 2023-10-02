package io.veronymous.client.jni;

import io.veronymous.client.exceptions.IllegalStateException;
import io.veronymous.client.exceptions.UnauthorizedException;

// TODO: Return if subscription is required
public class ConnectResult {

    private final String vpnConnection;
    private final boolean hasError;
    private final String error;
    private final boolean authRequired;
    private final String clientState;
    private final ServersStateResult serversStateResult;

    public ConnectResult(String vpnConnection,
                         boolean hasError,
                         String error,
                         boolean authRequired,
                         String clientState,
                         ServersStateResult serversStateResult) {
        this.vpnConnection = vpnConnection;
        this.hasError = hasError;
        this.error = error;
        this.authRequired = authRequired;
        this.clientState = clientState;
        this.serversStateResult = serversStateResult;
    }

    public String getVpnConnection() {
        return vpnConnection;
    }

    public boolean hasError() {
        return hasError;
    }

    public String getError() throws IllegalStateException {
        if (!this.hasError)
            throw new IllegalStateException("Does not have error.");
        return error;
    }

    public boolean isAuthRequired() {
        return authRequired;
    }

    public String getClientState() throws IllegalStateException, UnauthorizedException {
        if (this.hasError)
            throw new IllegalStateException("Has an error.");

        if (this.authRequired)
            throw new UnauthorizedException("Authentication is required.");

        return clientState;
    }

    public ServersStateResult getServersStateResult() {
        return serversStateResult;
    }

}
