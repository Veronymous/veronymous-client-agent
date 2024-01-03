package io.veronymous.vpn.android.app.ui.config

import io.veronymous.vpn.android.app.R


class ServerConfigs {

    companion object {
        val SERVERS: Map<String, ServerConfig> = mapOf(
            "ca_tor" to ServerConfig("Toronto, Canada", R.drawable.flag_ca),
            "usa_ny" to ServerConfig("New York, USA", R.drawable.flag_us),
            "gb_london" to ServerConfig("London, England", R.drawable.flag_gb_eng),
            "aus_syd" to ServerConfig("Sydney, Australia", R.drawable.flag_au)
        )
    }
}