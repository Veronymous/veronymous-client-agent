package io.veronymous.client.exceptions;

public class JNIException extends VeronymousClientException {
    public JNIException() {
    }

    public JNIException(String message) {
        super(message);
    }

    public JNIException(String message, Throwable cause) {
        super(message, cause);
    }

    public JNIException(Throwable cause) {
        super(cause);
    }
}
