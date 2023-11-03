package io.veronymous.android.veronymous.client.exceptions;

import io.veronymous.client.exceptions.VeronymousClientException;

public class VeronymousParseException extends VeronymousClientException {
    public VeronymousParseException() {
    }

    public VeronymousParseException(String message) {
        super(message);
    }

    public VeronymousParseException(String message, Throwable cause) {
        super(message, cause);
    }

    public VeronymousParseException(Throwable cause) {
        super(cause);
    }
}
