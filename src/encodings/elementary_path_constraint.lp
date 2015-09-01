
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


function strconc(x,y)
  return tostring(x)..":"..tostring(y)
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
countvertices(C+100)    :- C = #count{vertex(X)}.

% in network n exist a pos_path from X to Y
pos_path(E,gen(X),gen(Y),1, @strconc(X,Y)) :-
       	input(E,gen(X)), elabel(gen(X),gen(Y), 1), not input(E,gen(Y)),
       	vertex(gen(Y)).
pos_path(E,X,gen(Y), D+1,@strconc(P,Y))    :-
       	pos_path(E,X,Z,D,P), elabel(Z,gen(Y), 1), not input(E,gen(Y)),
       	vertex(gen(Y)),
	0==@member(Y,P), D<C, countvertices(C).     
                          
pos_path(E,X,gen(Y), D+1,@strconc(P,Y))    :-
       	neg_path(E,X,Z,D,P), elabel(Z,gen(Y),-1), not input(E,gen(Y)),
       	vertex(gen(Y)),
	0==@member(Y,P), D<C, countvertices(C).
                       
% in network n exist a neg_path from X to Y
neg_path(E,gen(X),gen(Y),1, @strconc(X,Y)) :-
       	input(E,gen(X)), elabel(gen(X),gen(Y),-1), not input(E,gen(Y)),
       	vertex(gen(Y)).
neg_path(E,X,gen(Y), D+1,@strconc(P,Y))    :-
       	neg_path(E,X,Z,D,P), elabel(Z,gen(Y), 1), not input(E,gen(Y)),
       	vertex(gen(Y)),
	0==@member(Y,P), D<C, countvertices(C).
                          
neg_path(E,X,gen(Y), D+1,@strconc(P,Y))    :-
       	pos_path(E,X,Z,D,P), elabel(Z,gen(Y),-1), not input(E,gen(Y)),
       	vertex(gen(Y)),
	0==@member(Y,P), D<C, countvertices(C).

% Y gets a positive influence
getspos(E,Y) :- input(E,X, 1), vertex(Y), pos_path(E,X,Y,_,P).
getspos(E,Y) :- input(E,X,-1), vertex(Y), neg_path(E,X,Y,_,P).

% Y gets a negative influence
getsneg(E,Y) :- input(E,X, 1), vertex(Y), neg_path(E,X,Y,_,P).
getsneg(E,Y) :- input(E,X,-1), vertex(Y), pos_path(E,X,Y,_,P).


% new inputs through repair
input(E,"unknownup",1) :- rep(new_influence(E,X,S)).
pos_path(E,"unknownup",X,1, @strconc("unknownup",X)) :- vlabel(E,X,1), rep(new_influence(E,X,1)).
neg_path(E,"unknownup",X,1, @strconc("unknownup",X)) :- vlabel(E,X,-1), rep(new_influence(E,X,-1)).


% labels must be consistent with depmat

 :- vlabel(E,V, 1), not getspos(E,V), not input(E,V).
 :- vlabel(E,V,-1), not getsneg(E,V), not input(E,V).

 :- vlabel(E,V,0), getspos(E,V), not getsneg(E,V).
 :- vlabel(E,V,0), getsneg(E,V), not getspos(E,V).

#hide.
