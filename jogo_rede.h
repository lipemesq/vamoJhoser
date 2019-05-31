//
// Biblioteca que contém a estrutura da mensagem, 
//	que é específica para aquela aplicação.
//  No geral, coisas que misturam o jogo com a rede.
//

#ifndef __TRON_REDE_FJ__
#define __TRON_REDE_FJ__


#include <iostream>
#include <vector>

// Endereço dos sockets
int socket; 
struct sockaddr_in eu = {0}, outro = {0};

// Estrutura da mensagem 
struct  str_mensagem {

	// Numero de sequencia da mensagem 
	unsigned int sequencia;
	
	// Bits de controle
	union {
		// Ler como inteiro por praticidade
		unsigned int _inteiro;
		
		// Ler como bitfield para economizar espaço
		struct {
			tipo : 1;
			sei_la : 2; 
			// Mais coisas
		} _bits;
	} controle;


}







#endif
