use crate::datastructures::*;
use crate::millgame::*;

use id_tree::{*, InsertBehavior::*};
use std::f32;
use rand::seq::SliceRandom;

pub fn monte_carlo_ts(gameboard:&GameBoard,player:Player)->MillMove {
    let mut tree: Tree<(f32, i32, GameBoard,u32)> =build_init_tree(&gameboard,player);
    let root =tree.get(&tree.root_node_id().unwrap()).unwrap();
    let root_id =tree.root_node_id().unwrap().clone();
    for i in 0..100 {
        let mut choosen_node=selection(&tree);
        let mut win_or_lose:f32=0.0;
        if tree.get(&choosen_node).unwrap().data().1==0{
            win_or_lose=playout(&choosen_node,&tree,player);
        }else {
            choosen_node=expand(&choosen_node,&mut tree, player);
            win_or_lose=playout(&choosen_node,&tree,player);
        } 
        backpropagation(choosen_node,&mut tree,player,win_or_lose,root_id.clone());
    }
    let move_amount = gameboard.possile_moves_vector(player);
    let mut  s_biggest :f32=0.0;
    let mut best_node_index=0;
    let mut best_node_index_counter=0;
    for child in tree.get(&tree.root_node_id().unwrap()).unwrap().clone().children(){
        let mut wx:f32=tree.get(&child).unwrap().data().0.try_into().unwrap();
        let mut nx:i32=tree.get(&child).unwrap().data().1.try_into().unwrap();
        let root_node =tree.get(tree.root_node_id().unwrap()).unwrap();
        let temp_calc =(root_node.data().1/nx) as f32;
        let mut s :f32=0.0;
        s= wx/nx as f32 + f32::sqrt(2.0) * f32::sqrt(f32::ln(temp_calc)/nx as f32);
        if s_biggest<= s{
            s_biggest=s;
            best_node_index=best_node_index_counter;
        }
        best_node_index_counter+=1
    }
    print!("{:?}",best_node_index);
    let temp_move = move_amount.get(best_node_index).unwrap().clone();
    temp_move
}

fn selection(tree:&Tree<(f32, i32, GameBoard,u32)>)->NodeId{
    let mut temp_Node: &Node<(f32,i32,GameBoard,u32)> =tree.get(tree.root_node_id().unwrap()).unwrap();
    let mut temp_NodeId: NodeId = tree.root_node_id().unwrap().clone();
    let mut s:f32=0.0;
    let mut s_biggest:f32=0.0;
    while !temp_Node.children().len()==0 {
        let mut iterator: LevelOrderTraversalIds<'_, (f32, i32, GameBoard,u32)>=tree.traverse_level_order_ids(&temp_NodeId).unwrap();
        loop {
            if iterator.next()==None {
                break;
            }
            let temp_iter_nodeId:NodeId =iterator.next().unwrap();
            let mut wx:f32=tree.get(&temp_iter_nodeId).unwrap().data().0.try_into().unwrap();
            let mut nx:i32=tree.get(&temp_iter_nodeId).unwrap().data().1.try_into().unwrap();
            //parent node 
            let mut parent =tree.get(tree.get(&temp_iter_nodeId).unwrap().parent().unwrap()).unwrap();
            let temp_calc =(parent.data().1/nx) as f32;
            s= wx/nx as f32 + f32::sqrt(2.0) * f32::sqrt(f32::ln(temp_calc)/nx as f32);
            if s_biggest<= s{
                s_biggest=s;
                temp_NodeId=temp_iter_nodeId;
            }
        }
    }

    temp_NodeId
}

fn expand(node:&NodeId,tree:&mut Tree<(f32, i32, GameBoard,u32)>,player:Player)->NodeId{
    let mut gameboard: GameBoard=tree.get(&node).unwrap().data().2.clone();
    let mut player_op;
    if tree.get(&node).unwrap().data().3 %2 ==0{
        player_op = get_other_player(player);
    }else {
        player_op =player.clone();
    }
    let mut all_moves: Vec<MillMove>= gameboard.possile_moves_vector(player_op);
    for moves in all_moves{
        let simulated_game: GameBoard =gameboard.move_simulator(moves);
        tree.insert(Node::new((0.0,1,simulated_game,tree.get(&node).unwrap().data().3+1)), UnderNode(&node)).unwrap();
    }
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let mut child_vec = tree.get(&node).unwrap().children().clone();
    let mut temp = node.clone();
    if let Some(random_element) = child_vec.choose_mut(&mut rng){
        temp=random_element.clone();
    }
    temp
}
fn playout(node:&NodeId,tree:&Tree<(f32, i32, GameBoard,u32)>,player:Player)->f32{
    let mut game = MillGame::new();
    let mut player_op;
    if tree.get(&node).unwrap().data().3 %2 ==0{
        player_op = get_other_player(player);
    }else {
        player_op =player.clone();
    }
    let win_or_lose= game.random_playout(tree.get(&node).unwrap().data().2.clone(),player_op,player);
    win_or_lose
}   
fn backpropagation(node:NodeId,mut tree:&mut Tree<(f32, i32, GameBoard,u32)>,player:Player, win_or_lose:f32,root:NodeId){
    let mut temp_node_id = node;
    loop{
        if &temp_node_id == tree.root_node_id().unwrap(){
            tree.get_mut(&root).unwrap().data_mut().0 +=win_or_lose;
            tree.get_mut(&root).unwrap().data_mut().1 +=1;
            break
        }
        tree.get_mut(&temp_node_id).unwrap().data_mut().0 +=win_or_lose;
        tree.get_mut(&temp_node_id).unwrap().data_mut().1 +=1;
        temp_node_id=tree.get(&temp_node_id).unwrap().parent().unwrap().clone();
    }
}
fn build_init_tree(gameboard:&GameBoard,player:Player)->Tree<(f32,i32,GameBoard,u32)>{
    let mut tree: Tree<(f32,i32,GameBoard,u32)> = TreeBuilder::new().build();
    let root: NodeId = tree.insert(Node::new((0.0,1,gameboard.clone(),0)), AsRoot).unwrap();
    let all_moves: Vec<MillMove>= gameboard.possile_moves_vector(player);
    for moves in all_moves{
        let simulated_game: GameBoard =gameboard.move_simulator(moves);
        tree.insert(Node::new((0.0,1,simulated_game,1)), UnderNode(&root)).unwrap();
    }
    tree
}
