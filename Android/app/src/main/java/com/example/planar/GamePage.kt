package com.example.planar

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.unit.dp
//import androidx.compose.ui.unit.sp
import coil.compose.AsyncImage
import com.example.planar.shared_types.Event

@Composable
fun GamePage(core: Core, resourceHelper: ResourceHelper) {
    Row(
        verticalAlignment = Alignment.CenterVertically,
        modifier = Modifier
            .fillMaxSize()
            .padding(10.dp),
    ) {
        Column {
            Button(
                onClick = { /*Do Something~*/ },
                colors = ButtonDefaults.buttonColors(containerColor = MaterialTheme.colorScheme.primary)
            ) { Text(text = "Roll Dice", color = Color.White) }
            Button(
                onClick = {core.update(Event.ShuffleActive())},
                colors = ButtonDefaults.buttonColors(containerColor = MaterialTheme.colorScheme.primary)
            ) { Text(text = "Shuffle", color = Color.White) }
        }
        //Center Image
        Column(
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.Center,
            modifier = Modifier
                .fillMaxSize()
                .weight(1f)
        ) {
            AsyncImage(
                model = resourceHelper.getImage(core.view?.active_card.toString()),
                contentDescription = "test image",
                contentScale = ContentScale.Fit,
                modifier = Modifier
                    .fillMaxSize()
                    .weight(1f)
            )
        }
        Column{
            Button(
                onClick = {core.update(Event.NextCard())},
                colors = ButtonDefaults.buttonColors(containerColor = MaterialTheme.colorScheme.primary)
            ) { Text(text = "Planeswalk", color = Color.White) }
            Button(
                onClick = {core.update(Event.PreviousCard())},
                colors = ButtonDefaults.buttonColors(containerColor = MaterialTheme.colorScheme.primary)
            ) { Text(text = "Oh shit, go back", color = Color.White) }
        }
    }
}