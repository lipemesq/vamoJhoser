//
// Biblioteca que contém a estrutura da mensagem, 
//	as funções para desenhar as coisas na tela 
//  e enviar as mensagens
//

#ifndef __REDE_MSGS_FJ__
#define __REDE_MSGS_FJ__

#include "jogo_rede.h"

#include <iostream>
#include <vector>


// Ponteiro para o tipo da mensagem 
typedef struct str_mensagem* mensagem_t;


// Envia a mensagem 'msg'
// Retorna 0 em caso de sucesso
int EnviarMensagem (mensagem_t* msg, int tamanho_mensagem, int socket, (struct sockaddr*) destino);


// Recebe uma mensagem e coloca em 'msg'
// Retorna 0 em caso de sucesso
int RecebeMensagem (mensagem_t* msg, int tamanho_maximo_mensagem, int socket, (struct sockaddr*) remetente);








#endif