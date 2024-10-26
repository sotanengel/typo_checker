from collections import defaultdict
import re

def create_rust_dictionary_file(input_file, output_file):
    length_dict = defaultdict(list)

    # 辞書ファイルの読み込み
    with open(input_file, 'r', encoding='utf-8') as infile:
        for line in infile:
            # 各行をタブで分割し、1列目を取得
            first_column = line.split('\t')[0].strip()
            # 空白で分割して2つ以上の単語がない場合
            if len(first_column.split()) == 1:
                # カンマで分割し、各単語をリストに追加
                words = [word.strip() for word in first_column.split(',')]
                for word in words:
                    # 単語がアルファベットのみの場合のみ追加
                    if word and re.match(r'^[a-zA-Z]+$', word):
                        length_dict[len(word)].append(word)

    # 各長さのリストの最大数を求める
    max_length = max(len(group) for group in length_dict.values())

    # 固定長の配列に収めるための最大長さを設定
    result = []

    # 長さごとにリストを作成し、文字数の順にソート
    for key in sorted(length_dict):
        sorted_words = sorted(length_dict[key])  # ソート
        # 空の文字列の代わりにNoneを使用
        while len(sorted_words) < max_length:
            sorted_words.append(None)  # Noneで埋める
        result.append(sorted_words[:max_length])  # 固定長の配列に追加

    # Rustファイルの生成
    with open(output_file, 'w', encoding='utf-8') as outfile:
        outfile.write('pub fn get_dictionary() -> [[Option<&\'static str>; {}]; {}] {{\n'.format(max_length, len(result)-1))
        outfile.write('    [\n')
        for group in result[1:]:
            outfile.write('        [\n')
            for word in group:
                if word is not None:
                    outfile.write(f'            Some("{word}"),\n')
                else:
                    outfile.write('            None,\n')
            outfile.write('        ],\n')
        outfile.write('    ]\n')
        outfile.write('}\n')

# 実行部分
input_file = '20241025_ejdict-hand-utf8.txt'  # 入力ファイル名
output_file = 'dictionary.rs'  # 出力ファイル名
create_rust_dictionary_file(input_file, output_file)
