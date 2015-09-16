%special case: check elementary path consistency with remove edges and bound add edges

0{rep(remedge(U,V,S))}1 :- not mandatory(U,V), obs_elabel(U,V,S).
0{rep(remedge(U,V,1)), rep(remedge(U,V,-1))}2 :- not mandatory(U,V), edge(U,V), not obs_elabel(U,V,1), not obs_elabel(U,V,-1).

% guess edges to add
0{rep(addedge(U,V,1)),rep(addedge(U,V,-1))}1 :- vertex(U), vertex(V), U!=V,
                                               not obs_elabel(U,V,1), not obs_elabel(U,V,-1).

% labels for newly added edgesm
r_elabel(U,V,1) :- rep(addedge(U,V,1)).
r_elabel(U,V,-1) :-rep(addedge(U,V,-1)).

% max_add_edges(M). is given

:- C = #count{rep(addedge(U,V,S))}, max_add_edges(M), C>M.

% compute getspos getsneg

#begin_lua

function string:split( inSplitPattern, outResults )
  if not outResults then
    outResults = { }
  end
  local theStart = 1
  local theSplitStart, theSplitEnd = string.find( self, inSplitPattern, theStart )
  while theSplitStart do
    table.insert( outResults, string.sub( self, theStart, theSplitStart-1 ) )
    theStart = theSplitEnd + 1
    theSplitStart, theSplitEnd = string.find( self, inSplitPattern, theStart )
  end
  table.insert( outResults, string.sub( self, theStart ) )
  return outResults
end


function strconc(l,x)
  local myString = tostring(l)..":"..tostring(x)                                                  
  local myset = myString:split(":")
 
  table.sort(myset)
  retstring = ""
  for i,n in ipairs(myset) do 
    if n=="" then
            --    print("WHOOOPS   ",n)
    else
      retstring= retstring..":"..n
    end
  end
  return retstring
end


function Set (list)
  local set = {}
  for _, l in ipairs(list) do set[l] = true end
  return set
end

function member(x,l)
  local myString = tostring(l)
  local myset = Set(myString:split(":"))
  if myset[x] then 
    return 1
  else 
    return 0
  end
end

#end_lua.

% use number of vertices as upper bound for path length
countvertices(C+1)    :- C = #count{vertex(X)}.

% in network n exist a pos_e_path from X to Y
pos_e_path(E,gen(X),gen(Y),1, @strconc(X,Y),0) :-
        input(E,gen(X)), 
        elabel(gen(X),gen(Y), 1), 
        not input(E,gen(Y)),
        vertex(gen(Y)).
pos_e_path(E,X,gen(Y), D+1,@strconc(P,Y), RC)    :-
        pos_e_path(E,X,Z,D,P,RC), 
        elabel(Z,gen(Y), 1),
%         not pos_e_path(E,X,gen(Y),D2), ppl(D2), D2<D+1,      % there is no shorter path 
        not input(E,gen(Y)),
        vertex(gen(Y)),
        0==@member(Y,P), D<C, countvertices(C).     
                          
pos_e_path(E,X,gen(Y), D+1,@strconc(P,Y),RC)    :-
        neg_e_path(E,X,Z,D,P,RC), 
        elabel(Z,gen(Y),-1),
%         not pos_e_path(E,X,gen(Y),D2), ppl(D2), D2<D+1,      % there is no shorter path  
        not input(E,gen(Y)),
        vertex(gen(Y)),
        0==@member(Y,P), D<C, countvertices(C).
                       
% in network n exist a neg_e_path from X to Y
neg_e_path(E,gen(X),gen(Y),1, @strconc(X,Y),0) :-
        input(E,gen(X)), 
        elabel(gen(X),gen(Y),-1), 
        not input(E,gen(Y)),
        vertex(gen(Y)).
neg_e_path(E,X,gen(Y), D+1,@strconc(P,Y), RC)    :-
        neg_e_path(E,X,Z,D,P,RC), 
        elabel(Z,gen(Y), 1), 
%         not neg_e_path(E,X,gen(Y), D2), ppl(D2), D2<D+1,     % there is no shorter path
        not input(E,gen(Y)),
        vertex(gen(Y)),
        0==@member(Y,P), D<C, countvertices(C).
                          
neg_e_path(E,X,gen(Y), D+1,@strconc(P,Y),RC)    :-
        pos_e_path(E,X,Z,D,P,RC), 
        elabel(Z,gen(Y),-1), 
%         not neg_e_path(E,X,gen(Y), D2), ppl(D2), D2<D+1,     % there is no shorter path
        not input(E,gen(Y)),
        vertex(gen(Y)),
        0==@member(Y,P), D<C, countvertices(C).
        
% in network n exist a pos_e_path from X to Y over repair add edge (r_elabel)
pos_e_path(E,gen(X),gen(Y),1, @strconc(X,Y),1) :-
        input(E,gen(X)), 
        r_elabel(gen(X),gen(Y), 1),
        max_add_edges(M), M>0,
        not input(E,gen(Y)),
        vertex(gen(Y)).
pos_e_path(E,X,gen(Y), D+1,@strconc(P,Y), RC+1)    :-
        pos_e_path(E,X,Z,D,P,RC),
        r_elabel(Z,gen(Y), 1),
%         not pos_e_path(E,X,gen(Y),D2), ppl(D2), D2<D+1,      % there is no shorter path
        max_add_edges(M), M>RC,                              % the maximum number of r_elabel is not reached
        not input(E,gen(Y)),
        vertex(gen(Y)),
        0==@member(Y,P), D<C, countvertices(C).     
                          
pos_e_path(E,X,gen(Y), D+1,@strconc(P,Y),RC+1)    :-
        neg_e_path(E,X,Z,D,P,RC),
        r_elabel(Z,gen(Y),-1),
%         not pos_e_path(E,X,gen(Y), D2), ppl(D2), D2<D+1,     % there is no shorter path
        max_add_edges(M), M>RC,                              % the maximum number of r_elabel is not reached
        not input(E,gen(Y)),
        vertex(gen(Y)),
        0==@member(Y,P), D<C, countvertices(C).
                       
% in network n exist a neg_e_path from X to Y over repair add edge (r_elabel)
neg_e_path(E,gen(X),gen(Y),1, @strconc(X,Y),1) :-
        input(E,gen(X)), 
        r_elabel(gen(X),gen(Y),-1),
        max_add_edges(M), M>0, 
        not input(E,gen(Y)),
        vertex(gen(Y)).
neg_e_path(E,X,gen(Y), D+1,@strconc(P,Y), RC+1)    :-
        neg_e_path(E,X,Z,D,P,RC),
        r_elabel(Z,gen(Y), 1),
%         not neg_e_path(E,X,gen(Y), D2), ppl(D2), D2<D+1,     % there is no shorter path
        max_add_edges(M), M>RC,                              % the maximum number of r_elabel is not reached
        not input(E,gen(Y)),
        vertex(gen(Y)),
        0==@member(Y,P), D<C, countvertices(C).
                          
neg_e_path(E,X,gen(Y), D+1,@strconc(P,Y),RC+1)    :-
        pos_e_path(E,X,Z,D,P,RC),
        r_elabel(Z,gen(Y),-1),
%         not neg_e_path(E,X,gen(Y), D2), ppl(D2), D2<D+1,     % there is no shorter path
        max_add_edges(M), M>RC,                              % the maximum number of r_elabel is not reached
        not input(E,gen(Y)),
        vertex(gen(Y)),
        0==@member(Y,P), D<C, countvertices(C).       

% project the path predicates only with pathlength
% pos_e_path(E,X,Y, D) :- pos_e_path(E,X,Y, D,P,RC).
% neg_e_path(E,X,Y, D) :- neg_e_path(E,X,Y, D,P,RC).
%possible path lengths
% ppl(1..C) :- countvertices(C).
        
% new inputs through repair
input(E,gen("unknownup")) :- rep(new_influence(E,X,S)).
obs_vlabel(E,gen("unknownup"),1) :- rep(new_influence(E,X,S)).  
elabel(gen("unknownup"), X,1)  :- rep(new_influence(E,X,1)).     
elabel(gen("unknownup"), X,-1) :- rep(new_influence(E,X,-1)).     


% Y gets a positive influence
getspos(E,Y) :- input(E,X), vlabel(E,X, 1), vertex(Y), pos_e_path(E,X,Y,_,_,_).
getspos(E,Y) :- input(E,X), vlabel(E,X,-1), vertex(Y), neg_e_path(E,X,Y,_,_,_).

% Y gets a negative influence
getsneg(E,Y) :- input(E,X), vlabel(E,X, 1), vertex(Y), neg_e_path(E,X,Y,_,_,_).
getsneg(E,Y) :- input(E,X), vlabel(E,X,-1), vertex(Y), pos_e_path(E,X,Y,_,_,_).


% labels must be consistent with depmat

 :- vlabel(E,V, 1), not getspos(E,V), not input(E,V).
 :- vlabel(E,V,-1), not getsneg(E,V), not input(E,V).

 :- vlabel(E,V,0), getspos(E,V), not getsneg(E,V).
 :- vlabel(E,V,0), getsneg(E,V), not getspos(E,V).

#hide.