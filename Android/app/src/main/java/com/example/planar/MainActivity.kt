package com.example.planar

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
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
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.lifecycle.viewmodel.compose.viewModel
import com.example.planar.shared_types.Event
import com.example.planar.ui.theme.PlanarTheme
import kotlinx.coroutines.launch

//This was the same across every example, should probably be left alone for now.
class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            PlanarTheme {
                Surface(
                    modifier = Modifier.fillMaxSize(), color = MaterialTheme.colorScheme.background
                ) {
                    View()
                }
            }
        }
    }
}

@Composable
fun View(core: Core = viewModel()) {

    Row(
        verticalAlignment = Alignment.CenterVertically,
        modifier = Modifier
            .fillMaxSize()
            .padding(10.dp),
    ) {
        //Left Button
        Button(
            onClick = { core.update(Event.Increment()) },
            colors = ButtonDefaults.buttonColors(containerColor = MaterialTheme.colorScheme.primary)
        ) { Text(text = "Increment", color = Color.White) }
        //Center Image
        Column(
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.Center,
            modifier = Modifier
                .fillMaxSize()
                .weight(1f)
        ) {
            core.view?.result?.let { Text(text = it, color = Color.White, fontSize = 30.sp, modifier = Modifier.padding(10.dp)) }
            Button(
                onClick = { core.update(Event.Reset()) }, colors = ButtonDefaults.buttonColors(
                    containerColor = MaterialTheme.colorScheme.error
                )
            ) { Text(text = "Reset", color = Color.White) }
        }
        //Right Button
        Button(
            onClick = { core.update(Event.Decrement()) },
            colors = ButtonDefaults.buttonColors(containerColor = MaterialTheme.colorScheme.secondary)
        ) { Text(text = "Decrement", color = Color.White) }
    }
}

@Preview(showBackground = true)
@Composable
fun DefaultPreview() {
    PlanarTheme {
        View()
    }
}