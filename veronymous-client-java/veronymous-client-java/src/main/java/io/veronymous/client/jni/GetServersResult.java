package io.veronymous.client.jni;

public class GetServersResult {

    private final String[] servers;
    private final ServersStateResult serversStateResult;


    public GetServersResult(String[] servers, ServersStateResult serversStateResult) {
        this.servers = servers;
        this.serversStateResult = serversStateResult;
    }


    public String[] getServers() {
        return servers;
    }

    public ServersStateResult getServersStateResult() {
        return serversStateResult;
    }

}
