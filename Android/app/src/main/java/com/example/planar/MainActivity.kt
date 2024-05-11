package com.example.planar

import android.content.res.Resources
import android.os.Bundle
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.annotation.DrawableRes
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.lifecycle.viewModelScope
import androidx.lifecycle.viewmodel.compose.viewModel
import com.example.planar.shared_types.Event
import com.example.planar.ui.theme.PlanarTheme
import kotlinx.coroutines.launch
import coil.compose.rememberAsyncImagePainter
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.painterResource
import coil.compose.AsyncImage
import coil.request.ImageRequest
import com.example.planar.ResourceHelper

class MainActivity : ComponentActivity() {

    private val helper: ResourceHelper = ResourceHelper()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        QuickCore(getPath(), helper)
        setContent {
            PlanarTheme {
                Surface(
                    modifier = Modifier.fillMaxSize(), color = MaterialTheme.colorScheme.background
                ) {
                    View(resourceHelper = helper)
                }
            }
        }
    }

    private fun getPath(): String {
        //Ostensibly finding and parsing this list has less overhead than attempting to open a
        //connection.
        if("Card_Database" !in this.databaseList()){
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

            for (entry in helper.getSequence()){
                update(Event.SetImage(entry.key, entry.value.toByte()))
            }
        }
    }
}

@Composable
fun View(
    core: Core = viewModel(),
    resourceHelper: ResourceHelper
) {

    Row(
        verticalAlignment = Alignment.CenterVertically,
        modifier = Modifier
            .fillMaxSize()
            .padding(10.dp),
    ) {
        //Left Button
        Button(
            onClick = { core.update(Event.Write()) },
            colors = ButtonDefaults.buttonColors(containerColor = MaterialTheme.colorScheme.primary)
        ) { Text(text = "Write", color = Color.White) }
        //Center Image
        Column(
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.Center,
            modifier = Modifier
                .fillMaxSize()
                .weight(1f)
        ) {
            core.view?.result?.let { Text(text = it, color = Color.White, fontSize = 30.sp, modifier = Modifier.padding(10.dp)) }
            AsyncImage(model = resourceHelper.getImage("Academy at Tolaria West"),
                contentDescription = "test image",
                contentScale = ContentScale.Fit,
                modifier = Modifier
                    .fillMaxSize()
                    .weight(1f)
            )
            Button(
                onClick = { core.update(Event.Reset()) }, colors = ButtonDefaults.buttonColors(
                    containerColor = MaterialTheme.colorScheme.error
                )
            ) { Text(text = "Reset", color = Color.White) }
        }
        //Right Button
        Button(
            onClick = { core.update(Event.Read()) },
            colors = ButtonDefaults.buttonColors(containerColor = MaterialTheme.colorScheme.secondary)
        ) { Text(text = "Read", color = Color.White) }
    }
}

@Preview(showBackground = true)
@Composable
fun DefaultPreview() {
    PlanarTheme {
        View(resourceHelper = ResourceHelper())
    }
}