# Go Fish Game
__Game__  
+Deck: deck
+Player: player  
+Player: opponent  
+int: turn  

__Deck__  
+vec(card): cards  
+Card: draw()  

__Card__  
+rank  
+suit  

__Player__  
+int: Pairs = 0  
+Hand: playerHand  

__Hand__  
+vec(cards): cards  
+bool: checkCard(Card)  
+void: removeCard(Card)  