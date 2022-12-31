from itertools import islice
import numpy as np
import sys
import os

def load_data(data_path):
    """ xx """
    with open(data_path, 'r') as f:

        # read draws from first line
        first_line = f.readline()
        draws = [int(draw.strip()) for draw in first_line.split(',')]

        boards = list()
        while True:

            # skip blank line
            blank = f.readline()

            # look for next bingo block
            next_five_lines = list(islice(f, 5))
            if not next_five_lines:
                break

            # process new bingo block
            new_board = list()
            for row in next_five_lines:
                cleaned_row = [int(val.strip()) for val in row.split()]
                new_board.append(cleaned_row)
            boards.append(new_board)

    return draws, boards


def find_winning_board(draws, boards, state_of_play):
    """ xx """
    i = 0
    while True:

        # initialize next draw
        draw = draws[i]

        # update state of play
        for j, board in enumerate(boards):
            for k, row in enumerate(board):
                for l, entry in enumerate(row):
                    if entry == draw:
                        state_of_play[j][k][l] = 1

            # check columns for winner
            if max(np.sum(state_of_play[j], axis=0)) >= 5:
                win_state = state_of_play[j]
                winner = board
                return i, winner, win_state

            # check rows for winner
            if max(np.sum(state_of_play[j], axis=1)) >= 5:
                win_state = state_of_play[j]
                winner = board
                return i, winner, win_state

        # increment
        i += 1



def find_losing_board(draws, boards, state_of_play):
    """ xx """
    i = 0
    win_tracker = [0]*len(boards)
    while True:

        # initialize next draw
        draw = draws[i]

        # update state of play
        for j, board in enumerate(boards):
            for k, row in enumerate(board):
                for l, entry in enumerate(row):
                    if entry == draw:
                        state_of_play[j][k][l] = 1

            # check columns for winner
            if max(np.sum(state_of_play[j], axis=0)) >= 5:
                win_state = state_of_play[j]
                winner = board
                win_tracker[j] = 1

            # check rows for winner
            if max(np.sum(state_of_play[j], axis=1)) >= 5:
                win_state = state_of_play[j]
                winner = board
                win_tracker[j] = 1

            # check if last win
            if sum(win_tracker) == len(boards):
                return i, winner, win_state

        # increment
        i += 1


def compute_score(idx, draws, board, win_state):

    # last called number
    last_called_number = draws[idx]

    # sum of unmarked numbers on winning board
    unmarked_sum = 0.0
    for i in range(win_state.shape[0]):
        for j in range(win_state.shape[1]):
            if win_state[i][j] == 0:
                unmarked_sum += board[i][j]

    return unmarked_sum * last_called_number


if __name__ == '__main__':

    input_filename = sys.argv[1]
    data_path = os.path.join('04', input_filename)

    draws, boards = load_data(data_path)
    state_of_play = np.zeros((len(boards), 5, 5))

    idx, winner, win_state = find_winning_board(draws, boards, state_of_play)
    winning_score = compute_score(idx, draws, winner, win_state)
    print("Final winning score: {}".format(winning_score))

    idx, loser, win_state = find_losing_board(draws, boards, state_of_play)
    losing_score = compute_score(idx, draws, loser, win_state)
    print("Final losing score: {}".format(losing_score))
