#minimize{ 0@1,notfalse : not false;
           1@1, U,V,S : remedge(U,V,S);
           2@1, U,V,S : addedge(U,V,S);
           2@1, V     : addeddy(V) 
         }.
