package io.veronymous.android.veronymous.client.config;

public class VeronymousConfig {

    // 1 hour
    public static final long EPOCH_LENGTH = 3600;

    // 5 minutes
    public static final long EPOCH_BUFFER = 300;

    // 12 hours
    public static final long KEY_LIFETIME = 43200;


    private VeronymousConfig() {
        throw new IllegalStateException("Utility class.");
    }
}
