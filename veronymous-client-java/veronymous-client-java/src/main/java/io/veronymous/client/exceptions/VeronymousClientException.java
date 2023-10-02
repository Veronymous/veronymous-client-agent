package io.veronymous.client.exceptions;

public class VeronymousClientException extends Exception {
    public VeronymousClientException() {
    }

    public VeronymousClientException(String message) {
        super(message);
    }

    public VeronymousClientException(String message, Throwable cause) {
        super(message, cause);
    }

    public VeronymousClientException(Throwable cause) {
        super(cause);
    }
}
