package com.example.planar

import android.os.Bundle
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.lifecycle.viewModelScope
import com.example.planar.shared_types.Event
import com.example.planar.ui.theme.PlanarTheme
import com.novi.serde.Unsigned
import kotlinx.coroutines.launch

class MainActivity : ComponentActivity() {

    private val helper: ResourceHelper = ResourceHelper()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        Log.d("Main Activity", "Pre-core call")
        QuickCore(getPath(), helper)
        setContent {
            PlanarTheme {
                Surface(
                    modifier = Modifier.fillMaxSize(), color = MaterialTheme.colorScheme.background
                ) {
                    NavHost(resourceHelper = helper)
                }
            }
        }
    }

    private fun getPath(): String {
        //Ostensibly finding and parsing this list has less overhead than attempting to open a
        //connection.
        if ("Card_Database" !in this.databaseList()) {
            applicationContext.openOrCreateDatabase("Card_Database", 0, null)
        }

        return applicationContext.getDatabasePath("Card_Database").path
    }
}

class QuickCore(s: String, helper: ResourceHelper) : Core() {
    init {
        viewModelScope.launch {
            update(Event.Initialize())
            update(Event.SetDatabase(s))


            for (entry in helper.getSequence()) {
                //Try serializing?
                update(Event.SetImage(entry.key, entry.value as @Unsigned Byte?))
            }
        }
    }
}

@Preview(showBackground = true)
@Composable
fun DefaultPreview() {
    PlanarTheme {
        NavHost(resourceHelper = ResourceHelper())
    }
}