package com.example.planar

import androidx.compose.runtime.Composable
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavHostController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import com.example.planar.PageList.GAME_PAGE
import com.example.planar.PageList.HOME

object PageList {
    const val HOME = "home"
    const val DECK_BUILDER = "deckBuilder"
    const val GAME_PAGE = "gamePage"
}

@Composable
fun NavHost(
    navController: NavHostController = rememberNavController(),
    core: Core = viewModel(),
    resourceHelper: ResourceHelper
){
    NavHost(
        navController = navController,
        startDestination = GAME_PAGE,
    ){
        composable(HOME){
            Home(
                core,
                gameStart = {
                    navController.navigate(("gamePage"))
                },
            )
        }

//        composable(DECK_BUILDER){
//            DeckBuilder(
//
//            )
//        }

        composable(GAME_PAGE){
            GamePage(
                core,
                resourceHelper
            )
        }
    }
}
