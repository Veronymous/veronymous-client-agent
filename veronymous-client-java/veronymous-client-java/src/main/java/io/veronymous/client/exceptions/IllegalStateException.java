package io.veronymous.client.exceptions;

public class IllegalStateException extends VeronymousClientException {
    public IllegalStateException() {
    }

    public IllegalStateException(String message) {
        super(message);
    }

    public IllegalStateException(String message, Throwable cause) {
        super(message, cause);
    }

    public IllegalStateException(Throwable cause) {
        super(cause);
    }
}
