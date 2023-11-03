package io.veronymous.android.veronymous.client;

import android.content.Context;
import android.util.Log;

import java.io.File;
import java.time.Instant;
import java.util.Random;
import java.util.concurrent.Executor;
import java.util.concurrent.Executors;

import io.veronymous.android.veronymous.client.config.VeronymousConfig;
import io.veronymous.android.veronymous.client.exceptions.VeronymousIOException;
import io.veronymous.android.veronymous.client.exceptions.VpnConnectionException;
import io.veronymous.android.veronymous.client.listener.VeronymousTaskListener;
import io.veronymous.android.veronymous.client.status.AuthStatus;
import io.veronymous.android.veronymous.client.utils.IOUtils;
import io.veronymous.client.exceptions.IllegalStateException;
import io.veronymous.client.exceptions.VeronymousClientException;
import io.veronymous.client.jni.AuthenticateResult;
import io.veronymous.client.jni.ConnectResult;
import io.veronymous.client.jni.GetServersResult;
import io.veronymous.client.jni.VeronymousClientJni;

public class VeronymousClient {

    private static final String TAG = VeronymousClient.class.getSimpleName();

    private static final String CLIENT_STATE = "veronymous_client.json";

    private static final String SERVERS_STATE = "servers.json";

    private static final Executor EXECUTOR = Executors.newSingleThreadExecutor();

    private static final Random RANDOM = new Random();


    static {
        System.loadLibrary("veronymous_client_jni");
    }

    public static void refreshAuthToken(Context context, VeronymousTaskListener<AuthStatus> listener)
            throws VeronymousIOException {
        String clientState = getClientState(context);

        EXECUTOR.execute(() -> {
            AuthenticateResult result = VeronymousClientJni.refreshAuthToken(clientState);

            if (result.hasError()) {
                listener.onResult(AuthStatus.AUTHENTICATION_REQUIRED);
            } else if (result.isSubscriptionRequired()) {
                listener.onResult(AuthStatus.SUBSCRIPTION_REQUIRED);
            } else {
                // Save the updated client state
                try {
                    saveClientState(context, result.getClientState());
                    listener.onResult(AuthStatus.AUTHENTICATED);
                } catch (VeronymousIOException | IllegalStateException e) {
                    listener.onError(e);
                }
            }
        });
    }

    public static void authenticate(Context context,
                                    String username,
                                    String password,
                                    VeronymousTaskListener<AuthStatus> listener)
            throws VeronymousIOException {
        String clientState = getClientState(context);

        EXECUTOR.execute(() -> {
            AuthenticateResult result = VeronymousClientJni.authenticate(
                    username,
                    password,
                    clientState
            );

            if (result.hasError()) {
                listener.onResult(AuthStatus.AUTHENTICATION_REQUIRED);
            } else if (result.isSubscriptionRequired()) {
                listener.onResult(AuthStatus.SUBSCRIPTION_REQUIRED);
            } else {
                // Save the updated client state
                try {
                    saveClientState(context, result.getClientState());
                    listener.onResult(AuthStatus.AUTHENTICATED);
                } catch (VeronymousIOException | IllegalStateException e) {
                    listener.onError(e);
                }
            }
        });
    }

    public static void getServers(Context context, VeronymousTaskListener<String[]> listener)
            throws VeronymousIOException {
        String serversState = getServersState(context);

        EXECUTOR.execute(() -> {
            GetServersResult result = VeronymousClientJni.getServers(serversState);

            if (result.getServersStateResult().hasUpdate()) {
                try {
                    saveServersState(context, result.getServersStateResult().getServersState());
                } catch (VeronymousIOException | IllegalStateException e) {
                    listener.onError(e);
                    return;
                }
            }

            listener.onResult(result.getServers());
        });
    }

    public static void connect(Context context,
                               String server,
                               VeronymousTaskListener<String> listener)
            throws VeronymousIOException {
        String clientState = getClientState(context);
        String serversState = getServersState(context);

        EXECUTOR.execute(() -> {
            try {
                // Create VPN connection
                ConnectResult connectResult
                        = VeronymousClientJni.connect(server, clientState, serversState);

                // Attempt to connect 3 times
                int attempts = 0;
                while (connectResult.hasError() && attempts < 5) {
                    Log.d(
                            TAG,
                            "Got a connection error. Trying again in 1 second ",
                            new VpnConnectionException(connectResult.getError())
                    );
                    try {
                        Thread.sleep(1000);
                    } catch (InterruptedException e) {
                        throw new RuntimeException(e);
                    }
                    connectResult = VeronymousClientJni.connect(server, clientState, serversState);
                    attempts++;
                }

                if (connectResult.hasError()) {
                    listener.onError(new VpnConnectionException(connectResult.getError()));
                    return;
                }

                saveClientState(context, connectResult.getClientState());

                if (connectResult.getServersStateResult().hasUpdate())
                    saveServersState(context, connectResult.getServersStateResult().getServersState());

                listener.onResult(connectResult.getVpnConnection());
            } catch (VeronymousClientException e) {
                listener.onError(e);
            }

        });
    }

    /**
     * Get the time until the next refresh
     *
     * @return Time
     */
    public static long getTimeToRefresh() {
        long now = Instant.now().getEpochSecond();

        // Get the next epoch
        long nextEpoch = getNextEpoch(now, VeronymousConfig.EPOCH_LENGTH);

        // Check if currently in the buffer

        if (isInBuffer(now))
            // Go to the subsequent epoch
            nextEpoch += VeronymousConfig.EPOCH_LENGTH;

        // 10 second for time sync tolerance
        long bufferStart = nextEpoch - VeronymousConfig.EPOCH_BUFFER - now + 10;
        long bufferEnd = nextEpoch - now - 10;

        return RANDOM.nextInt((int) (bufferEnd - bufferStart + 1)) + bufferStart;
    }

    private static String getServersState(Context context) throws VeronymousIOException {
        File file = new File(context.getFilesDir(), SERVERS_STATE);

        if (!file.exists())
            return VeronymousClientJni.newServersState();
        else
            return IOUtils.readString(context, file);
    }

    private static void saveServersState(Context context, String serversState)
            throws VeronymousIOException {
        File file = new File(context.getFilesDir(), SERVERS_STATE);

        IOUtils.writeString(context, file, serversState);
    }

    private static String getClientState(Context context) throws VeronymousIOException {
        File file = new File(context.getFilesDir(), CLIENT_STATE);

        if (!file.exists())
            return VeronymousClientJni.newClientState();
        else
            return IOUtils.readString(context, file);
    }

    private static void saveClientState(Context context, String clientState)
            throws VeronymousIOException {
        File file = new File(context.getFilesDir(), CLIENT_STATE);

        IOUtils.writeString(context, file, clientState);
    }

    private static long getNextEpoch(long now, long epochLength) {
        // Current epoch start
        long currentEpoch = now - (now % epochLength);
        // Next epoch start
        long nextEpoch = currentEpoch + epochLength;

        long timeUntilNextEpoch = nextEpoch - now;

        return now + timeUntilNextEpoch;
    }

    private static boolean isInBuffer(long now) {
        return VeronymousConfig.EPOCH_BUFFER >
                (VeronymousConfig.EPOCH_LENGTH - (now % VeronymousConfig.EPOCH_LENGTH));
    }

}
