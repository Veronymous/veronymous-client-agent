package io.veronymous.android.vpn.app.ui.fragments.adapters

import android.content.Context
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.BaseAdapter
import android.widget.ImageView
import android.widget.TextView
import io.veronymous.android.vpn.app.R
import io.veronymous.android.vpn.app.ui.config.ServerConfigs

class ServerListAdapter(
    private val context: Context,
    private val servers: Array<String>
) : BaseAdapter() {

    private var selectedPosition = -1;

    override fun getCount(): Int {
        return this.servers.size
    }

    override fun getItem(position: Int): String {
        return this.servers[position]
    }

    override fun getItemId(position: Int): Long {
        return position.toLong()
    }

    override fun getView(position: Int, convertView: View?, parent: ViewGroup?): View {
        val view: View = if (convertView == null) {
            val inflater = LayoutInflater.from(this.context)
            inflater.inflate(R.layout.server_list_item, parent, false)
        } else
            convertView;

        val serverName = this.getItem(position)

        val serverConfig = ServerConfigs.SERVERS[serverName];


        val serverNameView = view.findViewById<TextView>(R.id.server_name)
        if (serverConfig != null) {
            serverNameView.text = serverConfig.displayName

            val flagView = view.findViewById<ImageView>(R.id.server_flag_view)
            flagView.setImageResource(serverConfig.flag)
        } else
            serverNameView.text = serverName


        // Set underline
        val underline = view.findViewById<View>(R.id.server_name_underline);
        if (this.selectedPosition == position)
            underline.visibility = View.VISIBLE;
        else
            underline.visibility = View.INVISIBLE


        return view;
    }

    fun setSelected(selectedPosition: Int) {
        this.selectedPosition = selectedPosition;
    }

}