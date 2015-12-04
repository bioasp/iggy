
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

function str(x)
  local myString = tostring(x)
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




% new inputs through repair
input(E,gen("unknownup"))    :- rep(new_influence(E,X,S)).
vlabel(E,gen("unknownup"),1) :- rep(new_influence(E,X,S)).
elabel(gen("unknownup"), X, 1)   :- rep(new_influence(E,X,1)).     
elabel(gen("unknownup"), X,-1)   :- rep(new_influence(E,X,-1)).     


% in a network exists under Condition E a positive path to X

pos_path(E,X,@str(X)) :- input(E,X), vlabel(E,X, 1).
neg_path(E,X,@str(X)) :- input(E,X), vlabel(E,X,-1).

pos_path(E,X,@strconc(P,X)) :- pos_path(E,Y,P), 
                               elabel(Y,X, 1), not input(E,X), X!=Y,
	                       0==@member(X,P).     
                          
neg_path(E,X,@strconc(P,X)) :- pos_path(E,Y,P), 
                               elabel(Y,X,-1), not input(E,X), X!=Y,
	                       0==@member(X,P).     

pos_path(E,X,@strconc(P,X)) :- neg_path(E,Y,P), 
                               elabel(Y,X,-1), not input(E,X), X!=Y,
	                       0==@member(X,P).     
                       
neg_path(E,X,@strconc(P,X)) :- neg_path(E,Y,P), 
                               elabel(Y,X, 1), not input(E,X), X!=Y,
	                       0==@member(X,P).     

pos_path(E,V) :- pos_path(E,V,P).
neg_path(E,V) :- neg_path(E,V,P).

forbidden(E,V, 1) :- exp(E), vertex(V), not pos_path(E,V), not input(E,V).
forbidden(E,V,-1) :- exp(E), vertex(V), not neg_path(E,V), not input(E,V).


