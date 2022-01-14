#minimize{ 0@2,notfalse : not false;
           1@2,(E,X)    : flip_node_sign_Plus_to_0(E,X);
           2@2,(E,X)    : flip_node_sign_Plus_to_Minus(E,X);
           1@2,(E,X)    : flip_node_sign_Minus_to_0(E,X);
           2@2,(E,X)    : flip_node_sign_Minus_to_Plus(E,X);
           1@2,(E,X)    : flip_node_sign_0_to_Plus(E,X);
           1@2,(E,X)    : flip_node_sign_0_to_Minus(E,X);
           2@2,(E,X)    : flip_node_sign_notMinus_to_Minus(E,X);
           2@2,(E,X)    : flip_node_sign_notPlus_to_Plus(E,X)
         }.