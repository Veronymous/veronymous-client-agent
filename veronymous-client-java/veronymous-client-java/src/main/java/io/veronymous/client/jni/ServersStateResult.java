package io.veronymous.client.jni;

import io.veronymous.client.exceptions.IllegalStateException;

public class ServersStateResult {

    private final boolean hasUpdate;
    private final String serversState;

    public ServersStateResult(boolean hasUpdate, String serversState) {
        this.hasUpdate = hasUpdate;
        this.serversState = serversState;
    }

    public boolean hasUpdate() {
        return this.hasUpdate;
    }

    public String getServersState() throws IllegalStateException {
        if (!this.hasUpdate())
            throw new IllegalStateException("There isn't an update.");

        return serversState;
    }
}
