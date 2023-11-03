package io.veronymous.android.veronymous.client.exceptions;

import io.veronymous.client.exceptions.VeronymousClientException;

public class VeronymousIOException extends VeronymousClientException {
    public VeronymousIOException() {
    }

    public VeronymousIOException(String message) {
        super(message);
    }

    public VeronymousIOException(String message, Throwable cause) {
        super(message, cause);
    }

    public VeronymousIOException(Throwable cause) {
        super(cause);
    }
}
