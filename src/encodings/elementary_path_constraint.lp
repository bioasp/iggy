
#begin_lua

function string:split( inSplitPattern, outResults )
  --print("split",self)
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
  local myset = myString:split(":")
  for i,n in ipairs(myset) do
 --   print(i,n)
    if n==tostring(x) then
      return 1
    else
      -- print("noteq")
    end
  end
  return 0
end

#end_lua.

% use number of vertices as upper bound for path length
countvertices(C+1)    :- C = #count{vertex(X)}.

% in network n exist a pos_e_path from X to Y
pos_e_path(E,X,Y,D, P) :- pos_e_path(E,X,Y,D, P,L).
neg_e_path(E,X,Y,D, P) :- neg_e_path(E,X,Y,D, P,L).

pos_e_path(E,X,Y,1, @strconc(X,Y),X) :-
       	input(E,X), elabel(X,Y, 1), not input(E,Y),
       	vertex(Y).
pos_e_path(E,X,Y, D+1,@strconc(P,Y),Z)    :-
       	pos_e_path(E,X,Z,D,P), elabel(Z,Y, 1), not input(E,Y),
       	vertex(Y),
	0==@member(Y,P), D<C, countvertices(C).     
                          
pos_e_path(E,X,Y, D+1,@strconc(P,Y),Z)    :-
       	neg_e_path(E,X,Z,D,P), elabel(Z,Y,-1), not input(E,Y),
       	vertex(Y),
	0==@member(Y,P), D<C, countvertices(C).
                       
% in network n exist a neg_e_path from X to Y
neg_e_path(E,X,Y,1, @strconc(X,Y),X) :-
       	input(E,X), elabel(X,Y,-1), not input(E,Y),
       	vertex(Y).
neg_e_path(E,X,Y, D+1,@strconc(P,Y),Z)    :-
       	neg_e_path(E,X,Z,D,P), elabel(Z,Y, 1), not input(E,Y),
       	vertex(Y),
	0==@member(Y,P), D<C, countvertices(C).
                          
neg_e_path(E,X,Y, D+1,@strconc(P,Y),Z)    :-
       	pos_e_path(E,X,Z,D,P), elabel(Z,Y,-1), not input(E,Y),
       	vertex(Y),
	0==@member(Y,P), D<C, countvertices(C).


% new inputs through repair
input(E,gen("unknownup")) :- rep(new_influence(E,X,S)).
obs_vlabel(E,gen("unknownup"),1):- rep(new_influence(E,X,S)).  
elabel(gen("unknownup"), X,1)  :- rep(new_influence(E,X,1)).     
elabel(gen("unknownup"), X,-1) :- rep(new_influence(E,X,-1)).     


% Y gets a positive influence
getspos(E,Y,L) :- input(E,X), vlabel(E,X, 1), vertex(Y), pos_e_path(E,X,Y,_,P,L).
getspos(E,Y,L) :- input(E,X), vlabel(E,X,-1), vertex(Y), neg_e_path(E,X,Y,_,P,L).

% Y gets a negative influence
getsneg(E,Y,L) :- input(E,X), vlabel(E,X, 1), vertex(Y), neg_e_path(E,X,Y,_,P,L).
getsneg(E,Y,L) :- input(E,X), vlabel(E,X,-1), vertex(Y), pos_e_path(E,X,Y,_,P,L).

getspos(E,Y) :- getspos(E,Y,L).
getsneg(E,Y) :- getsneg(E,Y,L).


% elementary path constraint every change needs an elementary path
forbidden(E,V, 1) :- exp(E), vertex(V), not getspos(E,V), not input(E,V).
forbidden(E,V,-1) :- exp(E), vertex(V), not getsneg(E,V), not input(E,V).

depmat.