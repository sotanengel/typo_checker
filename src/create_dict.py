from collections import defaultdict

def create_rust_dictionary_file(input_file, output_file):
    length_dict = defaultdict(list)

    # 辞書ファイルの読み込み
    with open(input_file, 'r', encoding='utf-8') as infile:
        for line in infile:
            # 各行をタブで分割し、1列目を取得
            first_column = line.split('\t')[0].strip()
            if first_column:  # 空でない場合に処理
                # カンマで分割し、各単語をリストに追加
                words = [word.strip() for word in first_column.split(',')]
                for word in words:
                    length_dict[len(word)].append(word)

    # 長さごとにリストを作成し、文字数の順にソート
    result = [sorted(length_dict[key]) for key in sorted(length_dict)]

    # Rustファイルの生成
    with open(output_file, 'w', encoding='utf-8') as outfile:
        outfile.write('pub fn get_dictionary() -> Vec<Vec<&\'static str>> {\n')
        outfile.write('    vec![\n')
        for group in result:
            # グループ内の単語を文字列形式で出力
            outfile.write('        vec![\n')
            for word in group:
                outfile.write(f'            "{word}",\n')
            outfile.write('        ],\n')
        outfile.write('    ]\n')
        outfile.write('}\n')

# 実行部分
input_file = '20241025_ejdict-hand-utf8.txt'  # 入力ファイル名
output_file = 'dictionary.rs'  # 出力ファイル名
create_rust_dictionary_file(input_file, output_file)
