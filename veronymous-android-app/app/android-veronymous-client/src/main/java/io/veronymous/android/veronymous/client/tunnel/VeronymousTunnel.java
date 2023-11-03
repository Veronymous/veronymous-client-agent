package io.veronymous.android.veronymous.client.tunnel;

import android.util.Log;

import com.wireguard.android.backend.Tunnel;

@Deprecated
public class VeronymousTunnel implements Tunnel {

    private static final String TAG = VeronymousTunnel.class.getSimpleName();

    private static final String TUNNEL_NAME = "veron0";

    @Override
    public String getName() {
        return TUNNEL_NAME;
    }

    @Override
    public void onStateChange(State newState) {
        Log.d(TAG, String.format("State change. NEW STATE %S", newState));
    }
}
