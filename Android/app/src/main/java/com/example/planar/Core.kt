package com.example.planar

import android.app.Activity
import android.content.Context
import android.util.Log
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.setValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.ui.platform.LocalContext
import androidx.lifecycle.viewModelScope
import com.example.planar.shared.processEvent
import com.example.planar.shared.view
import com.example.planar.shared_types.Effect
import com.example.planar.shared_types.Event
import com.example.planar.shared_types.Request
import com.example.planar.shared_types.Requests
import com.example.planar.shared_types.ViewModel
import kotlinx.coroutines.launch


open class Core : androidx.lifecycle.ViewModel() {
    var view: ViewModel? by mutableStateOf(null)
        private set

    fun update(event: Event) {
        val effects = processEvent(event.bincodeSerialize())

        val requests = Requests.bincodeDeserialize(effects)
        for (request in requests) {
            processEffect(request)
        }
    }

    private fun processEffect(request: Request) {
        when (request.effect) {
            is Effect.Render -> {
                this.view = ViewModel.bincodeDeserialize(view())
            }
        }
    }
}
