package io.veronymous.android.veronymous.client.config;

public class VeronymousConfig {

    // 1 hour
    public static final long EPOCH_LENGTH = 3600;

    // 5 minutes
    public static final long EPOCH_BUFFER = 300;

    // Addresses that should be excluded from the tunnel
    public static final String[] OUT_OF_BAND_HOSTS = {
            "token-issuer.veronymous.io",
            "idp.veronymous.io"
    };

    private VeronymousConfig() {
        throw new IllegalStateException("Utility class.");
    }
}
