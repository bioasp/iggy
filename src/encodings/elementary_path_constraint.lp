
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
pos_e_path(E,gen(X),gen(Y),1, @strconc(X,Y)) :-
       	input(E,gen(X)), elabel(gen(X),gen(Y), 1), not input(E,gen(Y)),
       	vertex(gen(Y)).
pos_e_path(E,X,gen(Y), D+1,@strconc(P,Y))    :-
       	pos_e_path(E,X,Z,D,P), elabel(Z,gen(Y), 1), not input(E,gen(Y)),
       	vertex(gen(Y)),
	0==@member(Y,P), D<C, countvertices(C).     
                          
pos_e_path(E,X,gen(Y), D+1,@strconc(P,Y))    :-
       	neg_e_path(E,X,Z,D,P), elabel(Z,gen(Y),-1), not input(E,gen(Y)),
       	vertex(gen(Y)),
	0==@member(Y,P), D<C, countvertices(C).
                       
% in network n exist a neg_e_path from X to Y
neg_e_path(E,gen(X),gen(Y),1, @strconc(X,Y)) :-
       	input(E,gen(X)), elabel(gen(X),gen(Y),-1), not input(E,gen(Y)),
       	vertex(gen(Y)).
neg_e_path(E,X,gen(Y), D+1,@strconc(P,Y))    :-
       	neg_e_path(E,X,Z,D,P), elabel(Z,gen(Y), 1), not input(E,gen(Y)),
       	vertex(gen(Y)),
	0==@member(Y,P), D<C, countvertices(C).
                          
neg_e_path(E,X,gen(Y), D+1,@strconc(P,Y))    :-
       	pos_e_path(E,X,Z,D,P), elabel(Z,gen(Y),-1), not input(E,gen(Y)),
       	vertex(gen(Y)),
	0==@member(Y,P), D<C, countvertices(C).


% new inputs through repair
input(E,gen("unknownup")) :- rep(new_influence(E,X,S)).
obs_vlabel(E,gen("unknownup"),1):- rep(new_influence(E,X,S)).  
elabel(gen("unknownup"), X,1)  :- rep(new_influence(E,X,1)).     
elabel(gen("unknownup"), X,-1) :- rep(new_influence(E,X,-1)).     


% Y gets a positive influence
getspos(E,Y) :- input(E,X), vlabel(E,X, 1), vertex(Y), pos_e_path(E,X,Y,_,P).
getspos(E,Y) :- input(E,X), vlabel(E,X,-1), vertex(Y), neg_e_path(E,X,Y,_,P).

% Y gets a negative influence
getsneg(E,Y) :- input(E,X), vlabel(E,X, 1), vertex(Y), neg_e_path(E,X,Y,_,P).
getsneg(E,Y) :- input(E,X), vlabel(E,X,-1), vertex(Y), pos_e_path(E,X,Y,_,P).


mis_pos(E,and(Y)) :- elabel(Z,and(Y),1), not getspos(E,Y), exp(E).
mis_pos(E,and(Y)) :- elabel(Z,and(Y),-1), not getsneg(E,Y), exp(E).

mis_neg(E,and(Y)) :- elabel(Z,and(Y),1), not getsneg(E,Y), exp(E).
mis_neg(E,and(Y)) :- elabel(Z,and(Y),-1), not getspos(E,Y), exp(E).


% labels must be consistent with depmat

 :- vlabel(E,V, 1), not getspos(E,V), not input(E,V).
 :- vlabel(E,V,-1), not getsneg(E,V), not input(E,V).

 :- vlabel(E,V,0), getspos(E,V), not getsneg(E,V).
 :- vlabel(E,V,0), getsneg(E,V), not getspos(E,V).

#hide.
