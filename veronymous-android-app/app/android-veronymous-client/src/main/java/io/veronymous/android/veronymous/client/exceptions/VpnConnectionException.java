package io.veronymous.android.veronymous.client.exceptions;

import io.veronymous.client.exceptions.VeronymousClientException;

public class VpnConnectionException extends VeronymousClientException {
    public VpnConnectionException() {
    }

    public VpnConnectionException(String message) {
        super(message);
    }

    public VpnConnectionException(String message, Throwable cause) {
        super(message, cause);
    }

    public VpnConnectionException(Throwable cause) {
        super(cause);
    }
}
