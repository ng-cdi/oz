/* Generated file
   This file defines the search-path for added packs
*/

:- module(conf_packs, []).

:- multifile user:file_search_path/2.
:- dynamic user:file_search_path/2.
:- multifile cpack:registered_cpack/2.

:- cpack_register(pirates,'cpack/pirates',[home_url('https://cliopatria.swi-prolog.org/packs/pirates'),requires([])]).
