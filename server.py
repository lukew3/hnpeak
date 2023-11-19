from flask import Flask, render_template, request
from bs4 import BeautifulSoup
import requests
import sqlite3
import threading
import time

app = Flask(__name__)
con = sqlite3.connect("hnpeaks.db")

def get_peak(id):
    cur = con.cursor()
    res = cur.execute("SELECT peak_rank FROM posts WHERE id=?;", (id,))
    peak_rank = res.fetchone()
    return peak_rank

def background():
    con = sqlite3.connect("hnpeaks.db")
    scrape()

def scrape():
    cur = con.cursor()
    r = requests.get('https://news.ycombinator.com')
    soup = BeautifulSoup(r.text, 'html.parser')
    posts = soup.find_all('tr', class_='athing')
    for i, post in enumerate(list(posts)):
        print(post['id'], i+1)
        db = cur.execute("INSERT INTO posts (id, peak_rank, peak_time) VALUES (?, ?, ?);", (id, peak_rank, peak_time))
    time.sleep(120) # Wait 2 minutes to re-request
    scrape()

@app.route('/')
def main():
    id = request.args.get('id')
    if not id:
        return render_template('index.html')
    else:
        return render_template('index.html', title=title, position=position, time=time)


if __name__ == '__main__':
    x = threading.Thread(target=background)
    x.start()
    app.run()
