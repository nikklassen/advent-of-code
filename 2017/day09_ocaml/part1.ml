(* Happy Birthday to me *)
#use "topfind"
#require "batteries"

open Batteries

type parse_state = {
  ignore : bool ;
  score : int ;
  level : int ;
  garbage : bool ;
}

let create_parse_state () = { ignore = false ; score = 0 ; level = 0 ; garbage = false }

let try_read_line ic =
    try Some (input_line ic, ic) with End_of_file -> close_in ic; None

let compute_score state c =
  if state.ignore then
    { state with ignore = false }
  else
    match c with
    | '!' -> { state with ignore = true }
    | '{' when not state.garbage -> { state with level = state.level + 1 }
    | '}' when not state.garbage -> { state with score = state.score + state.level ; level = state.level - 1 }
    | '<' -> { state with garbage = true }
    | '>' -> { state with garbage = false }
    | _ -> state

let () =
    List.unfold (open_in "input") try_read_line
    |> List.map (fun s -> String.fold_left compute_score (create_parse_state ()) s |> fun s -> s.score)
    |> List.iter (fun s -> string_of_int s |> print_endline)
